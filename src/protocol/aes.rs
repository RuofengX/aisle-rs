use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt};

#[derive(Debug)]
pub struct Codec<C: BlockDecrypt + BlockEncrypt> {
    cipher: C,
}
impl<C: BlockDecrypt + BlockEncrypt> Codec<C> {
    pub fn new(cipher: C) -> Self {
        Self { cipher }
    }
    pub fn decode(&self, mut block: GenericArray<u8, C::BlockSize>) {
        self.cipher.decrypt_block(&mut block);
    }

    pub fn encode(&self, mut block: GenericArray<u8, C::BlockSize>) {
        self.cipher.encrypt_block(&mut block);
    }
}
