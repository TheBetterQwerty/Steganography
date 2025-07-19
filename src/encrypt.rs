#![allow(dead_code)]
use aes_gcm::{
    aead::Aead, Aes256Gcm, Key, Nonce, KeyInit
};
use sha2::{
    Sha256, Digest
};
use rand::random;

pub fn rand_bytes() -> Vec<u8> {
    let bytes: Vec<u8> = (0..12).map(|_| { random::<u8>() }).collect();
    bytes
}

pub fn hash(text: &[u8]) -> [u8; 32] {
    let res = Sha256::digest(text);
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&res);
    bytes
}

pub fn data_enc(key: &[u8], plaintext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(key);

    let ciphertext = match cipher.encrypt(nonce, plaintext) {
        Ok(x) => x,
        Err(x) => return Err(x.to_string())
    };

    Ok(ciphertext)
}

pub fn data_dec(key: &[u8], ciphertext: &[u8], nonce: &[u8]) -> Result<String, String> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(key);

    let plaintext = match cipher.decrypt(nonce, ciphertext) {
        Ok(x) => x,
        Err(x) => return Err(x.to_string())
    };

    Ok(String::from_utf8_lossy(&plaintext).to_string())
}
