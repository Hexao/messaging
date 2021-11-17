#[derive(Debug, Clone, PartialEq)]
pub struct UserList {
    users: Vec<(u32, String)>,
}

impl UserList {
    pub const ID: u8 = 0x10;

    pub fn new(users: Vec<(u32, String)>) -> Self {
        Self { users }
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        let slice_len = slice.len();

        // [list_len] => 1
        if slice_len < 1 {
            return Err(String::from("UserList must be at least 1 byte"));
        }

        let list_len = slice[0] as usize;

        let mut users = Vec::with_capacity(list_len);
        let mut cursor = 1;

        for _ in 0..list_len {
            if slice_len < cursor + 5 {
                return Err(String::from("UserList has incomplet data"));
            }

            let mut id = [0; 4];
            id.copy_from_slice(&slice[cursor..cursor + 4]);
            let id = u32::from_be_bytes(id);
            cursor += 4;

            let name_len = slice[cursor] as usize;
            cursor += 1;

            if slice_len < cursor + name_len {
                return Err(String::from("UserList has incomplet data"));
            }

            let name = std::str::from_utf8(
                &slice[cursor..cursor + name_len]
            ).unwrap().to_owned();

            cursor += name_len;
            users.push((id, name));
        }

        Ok(Self { users })
    }

    pub fn users(&self) -> &Vec<(u32, String)> {
        &self.users
    }

    pub fn msg_len(&self) -> usize {
        2 + self.users.iter().fold(0, |acc, (_ ,user)| {
            acc + user.len() + 5
        })
    }

    pub fn into_vec(self) -> Vec<u8> {
        let users_len = self.users.len();
        let mut vec = Vec::with_capacity(self.msg_len());

        vec.push(Self::ID);
        vec.push(users_len as u8);
        self.users.into_iter().fold(vec, |mut vec, (id, user)| {
            vec.extend_from_slice(&id.to_be_bytes());

            vec.push(user.len() as u8);
            vec.extend(user.into_bytes());

            vec
        })
    }
}
