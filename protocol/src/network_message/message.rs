#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    from: u32,
    content: String,
}

impl Message {
    pub const ID: u8 = 0x20;

    pub fn new(from: u32, content: String) -> Self {
        Self { from, content }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        let slice_len = slice.len();

        // [id_p0, id_p1, id_p2, id_p3, msg_len_up, msg_len_down, msg] => 6
        if slice_len < 7 {
            return Err(String::from("Message must be at least 7 byte"));
        }

        let mut from = [0; 4];
        from.copy_from_slice(&slice[..4]);
        let from = u32::from_be_bytes(from);

        let mut msg_len = [0; 2];
        msg_len.copy_from_slice(&slice[4..6]);
        let msg_len = u16::from_be_bytes(msg_len);

        if slice_len != 6 + msg_len as usize {
            return Err(String::from("Message has incomplete data"));
        }

        let content = std::str::from_utf8(
            &slice[6..]
        ).unwrap().to_owned();

        Ok(Self { from, content })
    }

    pub fn from(&self) -> u32 {
        self.from
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn msg_len(&self) -> usize {
        6 + self.content.len()
    }

    pub fn into_vec(self) -> Vec<u8> {
        let content_len = self.content.len();
        let mut vec = Vec::with_capacity(content_len + 6);

        vec.push(Self::ID);
        vec.extend_from_slice(&self.from.to_be_bytes());
        vec.extend_from_slice(&(content_len as u16).to_be_bytes());
        vec.extend(self.content.into_bytes());

        vec
    }
}
