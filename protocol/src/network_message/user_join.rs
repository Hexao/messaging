#[derive(Debug, Clone, PartialEq)]
pub struct UserJoin {
    name: String,
    id: u32,
}

impl UserJoin {
    pub const ID: u8 = 0x16;

    pub fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        let slice_len = slice.len();

        // [len, char, id_p0, id_p1, id_p2, id_p3] => 6
        if slice_len < 6 {
            return Err(String::from("UserJoin must be at least 6 byte"));
        }

        let name_len = slice[0] as usize;
        if slice_len != name_len + 5 {
            return Err(String::from("ServerList has incomplete data"));
        }

        let name = std::str::from_utf8(
            &slice[1..1 + name_len]
        ).unwrap().to_owned();

        let mut id = [0; 4];
        id.copy_from_slice(&slice[1 + name_len..]);
        let id = u32::from_be_bytes(id);

        Ok(Self { name, id })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn msg_len(&self) -> usize {
        self.name.len() + 6
    }

    pub fn into_vec(self) -> Vec<u8> {
        let name_len = self.name.len();
        let mut vec = Vec::with_capacity(name_len + 6);

        vec.push(Self::ID);
        vec.push(name_len as u8);
        vec.extend(self.name.into_bytes());
        vec.extend_from_slice(&self.id.to_be_bytes());

        vec
    }
}
