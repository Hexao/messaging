mod network_message;

pub mod multicast;
pub mod encrypt;

pub mod network {
    pub const MULTICAST_ADDRESS: &str = "233.141.56.26";
    pub const MULTICAST_PORT: u16 = 5358;

    pub use super::network_message::NetworkMessage;
}

#[cfg(test)]
mod slice_to_msg {
    use crate::network::NetworkMessage;

    #[test]
    fn client_identity() {
        let slice = &[0x4F, 0x04, 0x04, b'U', b's', b'e', b'r'];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::client_identity(
            String::from("User")
        ));
    }

    #[test]
    fn message() {
        let slice = &[0x4F, 0x20, 0x5E, 0x27, 0x44, 0xD2, 0x00, 0x0C, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd'];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::message(
            1_579_631_826,
            String::from("Hello, world")
        ));
    }

    #[test]
    fn personal_id() {
        let slice = &[0x4F, 0x1F, 0xD4, 0x25, 0x97, 0xE0];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::personal_id(3_559_233_504));
    }

    #[test]
    fn user_join() {
        let slice = &[0x4F, 0x16, 0x04, b'U', b's', b'e', b'r', 0xF1, 0x58, 0xB4, 0x49];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::user_join(
            String::from("User"),
            4_049_122_377
        ));
    }

    #[test]
    fn user_leave() {
        let slice = &[0x4F, 0x1A, 0x41, 0xDC, 0x3E, 0xAB];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::user_leave(
            1_104_953_003
        ));
    }

    #[test]
    fn user_list() {
        // empty
        let slice = &[0x4F, 0x10, 0x00];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::user_list(
            vec![]
        ));

        // len = 3
        let slice = &[0x4F, 0x10, 0x03,
            0x40, 0x00, 0x87, 0xCD, 0x06, b'U', b's', b'e', b'r', b'_', b'1',
            0x91, 0x02, 0x0D, 0x70, 0x06, b'U', b's', b'e', b'r', b'_', b'2',
            0x76, 0x54, 0xB7, 0xD2, 0x06, b'U', b's', b'e', b'r', b'_', b'3'
        ];
        let msg = NetworkMessage::from_slice(slice).unwrap();

        assert_eq!(msg, NetworkMessage::user_list(
            vec![
                (1_073_776_589, String::from("User_1")),
                (2_432_830_832, String::from("User_2")),
                (1_985_263_570, String::from("User_3"))
            ]
        ));
    }
}

#[cfg(test)]
mod msg_to_slice {
    use crate::network::NetworkMessage;

    #[test]
    fn client_identity() {
        let slice = [0x4F, 0x04, 0x04, b'U', b's', b'e', b'r'];

        assert_eq!(&slice[..], NetworkMessage::client_identity(
            String::from("User")
        ).into_vec());
    }

    #[test]
    fn message() {
        let slice = [0x4F, 0x20, 0x5E, 0x27, 0x44, 0xD2, 0x00, 0x0C, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd'];

        assert_eq!(&slice[..], NetworkMessage::message(
            1_579_631_826,
            String::from("Hello, world")
        ).into_vec());
    }

    #[test]
    fn personal_id() {
        let slice = [0x4F, 0x1F, 0xD4, 0x25, 0x97, 0xE0];

        assert_eq!(&slice[..], NetworkMessage::personal_id(3_559_233_504).into_vec());
    }

    #[test]
    fn user_join() {
        let slice = [0x4F, 0x16, 0x04, b'U', b's', b'e', b'r', 0xF1, 0x58, 0xB4, 0x49];

        assert_eq!(&slice[..], NetworkMessage::user_join(
            String::from("User"),
            4_049_122_377
        ).into_vec());
    }

    #[test]
    fn user_leave() {
        let slice = [0x4F, 0x1A, 0x41, 0xDC, 0x3E, 0xAB];

        assert_eq!(&slice[..], NetworkMessage::user_leave(
            1_104_953_003
        ).into_vec());
    }

    #[test]
    fn user_list() {
        // empty
        let slice = [0x4F, 0x10, 0x00];

        assert_eq!(&slice[..], NetworkMessage::user_list(
            vec![]
        ).into_vec());

        // len = 3
        let slice = [0x4F, 0x10, 0x03,
            0x40, 0x00, 0x87, 0xCD, 0x06, b'U', b's', b'e', b'r', b'_', b'1',
            0x91, 0x02, 0x0D, 0x70, 0x06, b'U', b's', b'e', b'r', b'_', b'2',
            0x76, 0x54, 0xB7, 0xD2, 0x06, b'U', b's', b'e', b'r', b'_', b'3'
        ];

        assert_eq!(&slice[..], NetworkMessage::user_list(
            vec![
                (1_073_776_589, String::from("User_1")),
                (2_432_830_832, String::from("User_2")),
                (1_985_263_570, String::from("User_3"))
            ]
        ).into_vec());
    }
}

#[cfg(test)]
mod key {
    #[test]
    pub fn key_exchange() {
        use rsa::pkcs1::{ToRsaPublicKey, FromRsaPublicKey};
        use rsa::RsaPublicKey;

        let (pub_key, _) = crate::encrypt::gen_key_pair().unwrap();
        let serialized_key = pub_key.to_pkcs1_pem().unwrap();

        println!("{}", serialized_key);
        let new_key = match RsaPublicKey::from_pkcs1_pem(&serialized_key) {
            Err(err) => panic!("{}", err),
            Ok(key) => key,
        };

        assert!(pub_key == new_key);
    }
}
