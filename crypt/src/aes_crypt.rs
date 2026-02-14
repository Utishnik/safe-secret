use aes_gcm_siv::{
    Aes256GcmSiv, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use debug_utils::{DEFAULT_FORMAT_RULE,VecString};

use crate::nonce_gen::gen_nonce;

#[derive(Debug)]
enum Error {
    RandErr,
    AesErr,
}

type Result<T> = core::result::Result<T, Error>;

use std::io::{Read, Write};

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const DATA_SIZE: usize = 1024;

use smallvec::SmallVec;

struct EncryptedFile {
    key: Aes256GcmSiv,
    nonce: Nonce,
    buffer: Vec<u8>,
}

impl EncryptedFile {
    fn new(key: &[u8; KEY_LEN]) -> Self {
        let cipher = Aes256GcmSiv::new_from_slice(key).unwrap();
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

        Self {
            key: cipher,
            nonce,
            buffer: Vec::new(),
        }
    }

    fn write_chunk(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let encrypted: core::result::Result<Vec<u8>, aes_gcm_siv::Error> =
            self.key.encrypt(&self.nonce, data);
        if encrypted.is_err(){
            return Err(Error::AesErr);
        }

        let unwrap_encrypted: Vec<u8> = encrypted.unwrap();

        let new_nonce: std::result::Result<[u8; NONCE_LEN], getrandom::Error> = gen_nonce();
        if new_nonce.is_err() {
            return Err(Error::RandErr);
        }

        let unwrap_new_nonce: [u8; NONCE_LEN] = new_nonce.unwrap();

        self.nonce = Nonce::from_slice(&unwrap_new_nonce).clone();

        Ok(unwrap_encrypted)
    }

    fn get_buffer(self) -> Vec<u8> {
        self.buffer
    }
    //todo идея хранить все в одном блоке память типо как расширенный стек
    fn encrypt_chunk(&mut self,key_bytes: [u8; KEY_LEN],data: SmallVec<[u8; DATA_SIZE]>) -> Result<Vec<u8>> {
       self.write_chunk(&data)
    }
}

#[test]
fn stream_encryption_test() {
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let key_bytes: [u8; KEY_LEN] = key.into();

    let mut encryptor: EncryptedFile = EncryptedFile::new(&key_bytes);

    let chunk1: &[u8; 10] = b"first  ...";
    let chunk2: &[u8; 10] = b"second ...";

    let encrypted1: Vec<u8> = encryptor.write_chunk(chunk1).unwrap();
    let encrypted2: Vec<u8> = encryptor.write_chunk(chunk2).unwrap();
    println!("encrypted1: {}",encrypted1.vec_string(DEFAULT_FORMAT_RULE));
    println!("encrypted1: {}",encrypted2.vec_string(DEFAULT_FORMAT_RULE));

}
