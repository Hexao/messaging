#[derive(Debug, Clone, PartialEq)]
pub struct MulticastMessage {
    content: Option<(String, u16)>,
}

impl MulticastMessage {
    pub fn ping() -> Self {
        Self { content: None }
    }

    pub fn server_identity(name: String, port: u16) -> Self {
        Self { content: Some((name, port)) }
    }

    pub fn is_ping(&self) -> bool {
        self.content.is_none()
    }

    pub fn content(&self) -> &Option<(String, u16)> {
        &self.content
    }
}

impl From<MulticastMessage> for Vec<u8> {
    fn from(msg: MulticastMessage) -> Self {
        match msg.content {
            Some((name, port)) => {
                let name_len = name.len();
                if name_len > u8::MAX as usize {
                    panic!("server name should be shorter than {} characters, found {}", u8::MAX, name_len);
                }

                let mut vec = Vec::with_capacity(name_len + 4);

                vec.push(0x4F);
                vec.push(name_len as u8);
                vec.extend(name.into_bytes());
                vec.extend_from_slice(&port.to_be_bytes());

                vec
            }
            None => vec![0x4F],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntoMulticastError {
    WrongStart,
    IncompleteData,
}

impl std::convert::TryInto<MulticastMessage> for &[u8] {
    type Error = IntoMulticastError;

    fn try_into(self) -> Result<MulticastMessage, Self::Error> {
        let slice_len = self.len();

        if slice_len < 1 {
            return Err(Self::Error::IncompleteData);
        }

        if self[0] == 0x4F {
            if slice_len > 1 {
                let name_len = self[1] as usize;

                if slice_len != name_len + 4 {
                    return Err(Self::Error::IncompleteData);
                }

                let name = std::str::from_utf8(
                    &self[2..2 + name_len]
                ).unwrap().to_owned();

                let mut port = [0; 2];
                port.copy_from_slice(&self[2 + name_len..]);
                let port = u16::from_be_bytes(port);

                Ok(MulticastMessage::server_identity(name, port))
            } else {
                Ok(MulticastMessage::ping())
            }
        } else {
            Err(Self::Error::WrongStart)
        }
    }
}

impl std::fmt::Display for MulticastMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.content {
            Some(_) => write!(f, "Server identity"),
            None => write!(f, "Ping"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;
    use super::MulticastMessage;

    #[test]
    fn ping() {
        let ping = MulticastMessage::ping();
        let vec: Vec<_> = ping.clone().into();

        let msg: MulticastMessage = vec
            .as_slice()
            .try_into()
            .unwrap();

        assert_eq!(ping, msg);
    }

    #[test]
    fn server_identity() {
        let si = MulticastMessage::server_identity(
            String::from("Server_name"), 4338
        );

        let vec: Vec<_> = si.clone().into();
        let msg: MulticastMessage = vec
            .as_slice()
            .try_into()
            .unwrap();

        assert_eq!(si, msg);
    }
}
