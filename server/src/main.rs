use std::{
    net::{Ipv4Addr, TcpListener, TcpStream, UdpSocket},
    io::{Read, Write},
    str::FromStr,
};
use protocol::{
    multicast::MulticastMessage,
    network::NetworkMessage,
};
use std::sync::{Arc, RwLock};
use std::convert::TryInto;
use rand::Rng;

fn main() {
    let name = match std::env::args().nth(1) {
        None => {
            println!("usage: server.exe <server_name>");
            return;
        }
        Some(name) => name,
    };

    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let handle_ping = std::thread::Builder::new().name(String::from("Ping")).spawn(move || {
        let thread_name = std::thread::current().name().unwrap_or("Unknown").to_owned();
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", protocol::network::MULTICAST_PORT)).unwrap();
        socket.join_multicast_v4(
            // multicast address must be between
            // 224.x.x.x and 239.x.x.x => D class
            &Ipv4Addr::from_str(protocol::network::MULTICAST_ADDRESS).unwrap(),
            &Ipv4Addr::new(0, 0, 0, 0)
        ).unwrap();

        let server_identity: Vec<_> = MulticastMessage::server_identity(name.to_owned(), port).into();
        println!("{}: Ready", thread_name);
        let mut buf = [0; 32];

        loop {
            let (len, addr) = match socket.recv_from(&mut buf) {
                Ok(infos) => infos,
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            };

            let slice = &buf[..len];
            let msg: MulticastMessage = match slice.try_into() {
                Ok(ping) => ping,
                Err(err) => {
                    println!("{}: {:?}", thread_name, err);
                    continue;
                }
            };

            if msg.is_ping() {
                println!("{}: Ping from {}", thread_name, addr);
                socket.send_to(&server_identity, addr).unwrap();
            } else {
                println!("{}: Unexpected {}", thread_name, msg);
            }
        }
    }).unwrap();

    let addr = Arc::new(RwLock::new(Vec::<(u32, String, TcpStream)>::with_capacity(50)));
    let (sender, receiver) = std::sync::mpsc::channel();
    let t_addr = addr.clone();

    let handle_tcp = std::thread::Builder::new().name(String::from("TCP")).spawn(move || {
        let thread_name = std::thread::current().name().unwrap_or("Unknown").to_owned();
        let mut rng = rand::thread_rng();

        loop {
            let mut stream = match listener.accept() {
                Ok((stream, _)) => stream,
                Err(err) => {
                    println!("{}: {}", thread_name, err);
                    break;
                }
            };

            let mut buf = [0; 32];
            stream.set_read_timeout(Some(std::time::Duration::from_secs(1))).unwrap();

            match stream.read(&mut buf) {
                Ok(len) => {
                    let msg = match NetworkMessage::from_slice(&buf[..len]) {
                        Ok(msg) => msg,
                        Err(err) => {
                            println!("{}: {}", thread_name, err);
                            continue;
                        }
                    };

                    if let NetworkMessage::ClientIdentity(client) = msg {
                        let addr_reader = t_addr.read().unwrap();
                        let users = addr_reader.iter().fold(vec![], |mut vec, (id, name, _)| {
                            vec.push((*id, name.to_owned()));
                            vec
                        });

                        let id = loop {
                            let new_id = rng.gen();

                            if addr_reader.iter().all(|(registered_id, ..)| new_id != *registered_id) {
                                break new_id;
                            }
                        };

                        drop(addr_reader);
                        stream.set_read_timeout(None).unwrap();
                        let sender = sender.clone();

                        let msg = NetworkMessage::personal_id(id).into_vec();
                        stream.write_all(&msg).unwrap();

                        std::thread::sleep(std::time::Duration::from_millis(125));

                        let msg = NetworkMessage::user_list(users).into_vec();
                        stream.write_all(&msg).unwrap();

                        sender.send(NetworkMessage::user_join(client.name().to_owned(), id)).unwrap();
                        t_addr.write().unwrap().push((id, client.name().to_owned(), stream.try_clone().unwrap()));

                        std::thread::Builder::new().name(format!("{}_thread", client.name())).spawn(move || {
                            let thread_name = std::thread::current().name().unwrap_or("Unknown").to_owned();
                            let mut buf = [0;1024];

                            loop {
                                let len = match stream.read(&mut buf) {
                                    Ok(len) => len,
                                    Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
                                        println!("{}: Closed by peer", thread_name);
                                        break;
                                    }
                                    Err(err) => {
                                        println!("{}: {}", thread_name, err);
                                        break;
                                    }
                                };

                                let slice = &buf[..len];
                                let msg = match NetworkMessage::from_slice(slice) {
                                    Ok(msg) => msg,
                                    Err(err) => {
                                        println!("{}: {}", thread_name, err);
                                        break;
                                    }
                                };

                                sender.send(msg).unwrap();
                            }

                            sender.send(NetworkMessage::user_leave(id)).unwrap();
                        }).unwrap();
                    } else {
                        println!("{}: Expected ClientIdentity, found {}", thread_name, msg);
                    }
                }
                Err(ref err) if err.kind() == std::io::ErrorKind::TimedOut => {
                    println!("{}: Expected ClientIdentity within second. Nothing happend", thread_name);
                }
                Err(err) => {
                    println!("{}: {}", thread_name, err);
                }
            };
        }
    }).unwrap();

    while let Ok(msg) = receiver.recv() {
        let buf = msg.clone().into_vec();

        match msg {
            NetworkMessage::UserJoin(join) => for (id, _, stream) in addr.write().unwrap().iter_mut() {
                if *id == join.id() {
                    continue;
                }

                stream.write_all(&buf).unwrap();
            },
            NetworkMessage::UserLeave(leave) => {
                let mut addr_lock = addr.write().unwrap();

                let remove = addr_lock.iter().position(|(id, _, _)| {
                    *id == leave.id()
                }).unwrap();
                addr_lock.remove(remove);

                for (_, _, stream) in addr_lock.iter_mut() {
                    stream.write_all(&buf).unwrap();
                }
            },
            NetworkMessage::Message(_) => for (_, _, stream) in addr.write().unwrap().iter_mut() {
                stream.write_all(&buf).unwrap();
            }
            _ => {}
        }
    }

    handle_ping.join().unwrap();
    handle_tcp.join().unwrap();
}
