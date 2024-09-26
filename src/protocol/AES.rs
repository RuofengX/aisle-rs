use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt};

pub struct Codec<C: BlockDecrypt + BlockEncrypt> {
    cipher: C,
}
impl<C: BlockDecrypt + BlockEncrypt> Codec<C> {
    fn decode(&self, mut block: GenericArray<u8, C::BlockSize>) {
        self.cipher.decrypt_block(&mut block);
    }

    fn encode(&self, mut block: GenericArray<u8, C::BlockSize>) {
        self.cipher.encrypt_block(&mut block);
    }
}
