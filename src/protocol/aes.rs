use std::error::Error;

use bytes::Bytes;
use soft_aes::aes::{aes_dec_ecb, aes_enc_block, aes_enc_ecb};

#[derive(Debug)]
pub struct Codec256 {
    key: [u8; 32],
}

impl Codec256 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn decode(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let result = aes_dec_ecb(&data, self.key.as_slice(), Some("0x80"))?;
        Ok(result)
    }

    pub fn encode(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let result = aes_enc_ecb(data, self.key.as_slice(), Some("0x80"))?;
        Ok(result)
    }
}
