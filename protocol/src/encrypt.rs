use aes::Aes128;
use rsa::{
    PaddingScheme, RsaPrivateKey, RsaPublicKey // PublicKey
};

use block_modes::{
    BlockMode, Cbc, block_padding::Pkcs7
};

use rand::{
    rngs::OsRng, RngCore,
};

use std::error::Error;
pub const KEY_LEN: usize = 16;
type SharedKey = Cbc<Aes128, Pkcs7>;

pub fn gen_key_pair() -> Result<(RsaPublicKey, RsaPrivateKey), Box<dyn Error>> {
    let mut rng = OsRng;

    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok((public_key, private_key))
}

pub fn gen_shared_key() -> ([u8; KEY_LEN], [u8; KEY_LEN]) {
    let mut rng = OsRng;

    let mut key = [0; KEY_LEN];
    let mut iv = [0; KEY_LEN];

    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    (key, iv)
}

pub fn make_shared_key(key: &[u8], iv: &[u8]) -> SharedKey {
    SharedKey::new_from_slices(key, iv).unwrap()
}

pub fn padding_scheme() -> PaddingScheme {
    PaddingScheme::new_pkcs1v15_encrypt()
}
