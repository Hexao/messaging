mod client_identity;
mod personal_id;
mod user_list;
mod user_join;
mod user_leave;
mod message;

use client_identity::ClientIdentity;
use personal_id::PersonalId;
use user_list::UserList;
use user_join::UserJoin;
use user_leave::UserLeave;
use message::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum NetworkMessage {
    // handshake
    // Ask4SharedKey(String),
    // NoSharedKey,
    // SharedKey(Vec<u8>),

    ClientIdentity(ClientIdentity),
    PersonalId(PersonalId),
    UserList(UserList),
    UserJoin(UserJoin),
    UserLeave(UserLeave),
    Message(Message),
}

impl NetworkMessage {
    const IDENTIFIER: u8 = 0x4F;

    pub fn client_identity(name: String) -> Self {
        Self::ClientIdentity(ClientIdentity::new(name))
    }

    pub fn personal_id(id: u32) -> Self {
        Self::PersonalId(PersonalId::new(id))
    }

    pub fn user_list(users: Vec<(u32, String)>) -> Self {
        Self::UserList(UserList::new(users))
    }

    pub fn user_join(name: String, id: u32) -> Self {
        Self::UserJoin(UserJoin::new(name, id))
    }

    pub fn user_leave(id: u32) -> Self {
        Self::UserLeave(UserLeave::new(id))
    }

    pub fn message(from: u32, content: String) -> Self {
        Self::Message(Message::new(from, content))
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() < 2 {
            return Err(String::from("Message must be at least 2 byte"));
        }

        if slice[0] != Self::IDENTIFIER {
            return Err(String::from("Message must start with 0x4F"));
        }

        match slice[1] {
            ClientIdentity::ID => Ok(Self::ClientIdentity(ClientIdentity::from_slice(&slice[2..])?)),
            PersonalId::ID => Ok(Self::PersonalId(PersonalId::from_slice(&slice[2..])?)),
            UserList::ID => Ok(Self::UserList(UserList::from_slice(&slice[2..])?)),
            UserJoin::ID => Ok(Self::UserJoin(UserJoin::from_slice(&slice[2..])?)),
            UserLeave::ID => Ok(Self::UserLeave(UserLeave::from_slice(&slice[2..])?)),
            Message::ID => Ok(Self::Message(Message::from_slice(&slice[2..])?)),

            unknown_id => Err(format!("Unknown identifier: {:#04X}", unknown_id)),
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        let (msg_len, data) = match self {
            NetworkMessage::ClientIdentity(ci) => (ci.msg_len(), ci.into_vec()),
            NetworkMessage::PersonalId(pi) => (pi.msg_len(), pi.into_vec()),
            NetworkMessage::UserList(ul) => (ul.msg_len(), ul.into_vec()),
            NetworkMessage::UserJoin(uj) => (uj.msg_len(), uj.into_vec()),
            NetworkMessage::UserLeave(ul) => (ul.msg_len(), ul.into_vec()),
            NetworkMessage::Message(ms) => (ms.msg_len(), ms.into_vec()),
        };

        let mut vec = Vec::with_capacity(msg_len + 1);
        vec.push(Self::IDENTIFIER);
        vec.extend(data);

        vec
    }
}

impl std::fmt::Display for NetworkMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            NetworkMessage::ClientIdentity(_) => "ClientIdentity",
            NetworkMessage::PersonalId(_) => "PersonalId",
            NetworkMessage::UserList(_) => "UserList",
            NetworkMessage::UserJoin(_) => "UserJoin",
            NetworkMessage::UserLeave(_) => "UserLeave",
            NetworkMessage::Message(_) => "Message",
        })
    }
}
