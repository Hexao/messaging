#[derive(Debug, Clone, PartialEq)]
pub struct ClientIdentity {
    name: String,
}

impl ClientIdentity {
    pub const ID: u8 = 0x04;

    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        let slice_len = slice.len();

        // [len, char] => 2
        if slice_len < 2 {
            return Err(String::from("PingServerList must be at least 2 byte"));
        }

        let name_len = slice[0] as usize;
        if slice_len != name_len + 1 {
            return Err(String::from("PingServerList has incomplete data"));
        }

        let user = std::str::from_utf8(
            &slice[1..1 + name_len]
        ).unwrap().to_owned();

        Ok(Self { name: user })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn msg_len(&self) -> usize {
        self.name.len() + 2
    }

    pub fn into_vec(self) -> Vec<u8> {
        let user_len = self.name.len();
        let mut vec = Vec::with_capacity(user_len + 2);

        vec.push(Self::ID);
        vec.push(user_len as u8);
        vec.extend(self.name.into_bytes());

        vec
    }
}
