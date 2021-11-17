#[derive(Debug, Clone, PartialEq)]
pub struct PersonalId {
    id: u32,
}

impl PersonalId {
    pub const ID: u8 = 0x1F;

    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        // [id_p0, id_p1, id_p2, id_p3] => 4
        if slice.len() != 4 {
            return Err(String::from("PersonalId must be 4 byte"));
        }

        let mut id = [0; 4];
        id.copy_from_slice(slice);
        let id = u32::from_be_bytes(id);

        Ok(Self { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn msg_len(&self) -> usize {
        5
    }

    pub fn into_vec(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.msg_len());

        vec.push(Self::ID);
        vec.extend_from_slice(&self.id.to_be_bytes());

        vec
    }
}
