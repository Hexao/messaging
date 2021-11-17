use super::incoming_messages::IncomingMessages;
use super::{Client, View, ClientMessage};

use protocol::network::NetworkMessage;

use std::io::Write;
use std::time::Duration;
use std::collections::HashMap;
use std::net::{UdpSocket, TcpStream, SocketAddr};

use iced::{
    Application, Subscription, Clipboard, Command,
};

impl Client {
    pub fn get_subscription(&self) -> Subscription<<Self as Application>::Message> {
        if let View::Chat { socket, .. } = &self.view {
            Subscription::from_recipe(IncomingMessages {
                stream: socket.try_clone().unwrap(),
            }).map(ClientMessage::IncomingMessages)
        } else {
            Subscription::none()
        }
    }

    pub fn update_ui(&mut self, message: <Self as Application>::Message, _clipboard: &mut Clipboard) -> Command<<Self as Application>::Message> {
        match message {
            ClientMessage::UpdateUsername(new_username) => {
                if let View::Home { .. } = &self.view {
                    self.username = new_username;
                }
            }
            ClientMessage::SubmitUsername => {
                if let View::Home { .. } = &self.view {
                    if self.username.is_empty() {
                        return Command::none();
                    }

                    let mut servers = vec![];
                    update_server_list(&mut servers);

                    self.view = View::SelectServer {
                        buttons: vec![],
                        servers,
                    };
                }
            }
            ClientMessage::RefreshServerList => {
                if let View::SelectServer { servers, .. } = &mut self.view {
                    update_server_list(servers);
                }
            }
            ClientMessage::SelectServer(socket) => {
                let buf = NetworkMessage::client_identity(self.username.to_owned()).into_vec();
                let mut socket = TcpStream::connect(socket).unwrap();
                socket.write_all(&buf).unwrap();

                self.view = View::Chat {
                    messages: Vec::with_capacity(50),
                    users: HashMap::default(),
                    scroll_view: iced::scrollable::State::default(),
                    input: iced::text_input::State::default(),
                    socket,
                    personal_id: 0,
                    message: String::default(),
                };
            }
            ClientMessage::UpdateMessage(msg) => {
                if let View::Chat { message, .. } = &mut self.view {
                    *message = msg;
                }
            }
            ClientMessage::SendMessage => {
                if let View::Chat { message, socket, personal_id, .. } = &mut self.view {
                    if message.is_empty() {
                        return Command::none();
                    }

                    let mut send = String::with_capacity(50);
                    std::mem::swap(message, &mut send);

                    let buf = NetworkMessage::message(*personal_id, send).into_vec();
                    socket.write_all(&buf).unwrap();
                }
            }
            ClientMessage::IncomingMessages(msg) => {
                if let View::Chat { users, messages, personal_id, .. } = &mut self.view {
                    match &msg {
                        NetworkMessage::PersonalId(pid) => {
                            *personal_id = pid.id();
                        }
                        NetworkMessage::UserList(list) => {
                            for (id, user) in list.users() {
                                users.insert(*id, user.to_owned());
                            }

                            messages.push((msg, String::default()));
                        }
                        NetworkMessage::UserJoin(join) => {
                            users.insert(join.id(), join.name().to_owned());
                            messages.push((msg, String::default()));
                        }
                        NetworkMessage::UserLeave(leave) => {
                            let user = users.remove(&leave.id()).unwrap();
                            messages.push((msg, user));
                        }
                        NetworkMessage::Message(m) => {
                            let from = m.from();

                            let user = if from == *personal_id {
                                self.username.to_owned()
                            } else {
                                users.get(&from).unwrap().to_owned()
                            };

                            messages.push((msg, user));
                        }
                        _ => {}
                    }
                }
            }
        }

        Command::none()
    }
}

fn update_server_list(servers: &mut Vec<(String, SocketAddr)>) {
    use protocol::multicast::MulticastMessage;

    servers.clear();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let buf: Vec<_> = MulticastMessage::ping().into();

    let addr = format!("{}:{}", protocol::network::MULTICAST_ADDRESS, protocol::network::MULTICAST_PORT);

    socket.set_read_timeout(Some(Duration::from_millis(150))).unwrap();
    socket.send_to(&buf, addr).unwrap();
    let mut buf = [0; 64];

    while let Ok((buf_len, mut addr)) = socket.recv_from(&mut buf) {
        let message: MulticastMessage = match buf[..buf_len].try_into() {
            Err(msg) => {
                println!("{:?}", msg);
                continue;
            }
            Ok(message) => message,
        };

        if let Some((name, port)) = message.content() {
            addr.set_port(*port);
            servers.push((name.to_owned(), addr));
        }
    }

    servers.sort_by(|(lhs, _), (rhs, _)| {
        let mut lhs = lhs.chars();
        let mut rhs = rhs.chars();

        loop {
            let cple = (lhs.next(), rhs.next());

            match cple {
                (Some(l), Some(r)) => {
                    let ord = l.to_lowercase().cmp(r.to_lowercase());
                    if !ord.is_eq() {
                        break ord;
                    }
                }
                (Some(_), None) => break std::cmp::Ordering::Greater,
                (None, Some(_)) => break std::cmp::Ordering::Less,
                (None, None) => break std::cmp::Ordering::Equal,
            }
        }
    });
}
