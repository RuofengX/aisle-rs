use std::future::Future;
use std::io;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockSizeUser, KeyInit};
use aes::Aes256;
use bytes::{Bytes, BytesMut};
use monoio::buf::IoBufMut;
use monoio::io::{AsyncBufRead, AsyncReadRent, AsyncReadRentExt, AsyncWriteRent, BufReader};
use monoio::net::TcpStream;
use monoio::BufResult;
use quick_impl::QuickImpl;

use crate::common::Meta;
use crate::error::{CodecError, ProtocolError};
use crate::{protocol, Error};

pub trait DuplexStream: AsyncReadRent + AsyncWriteRent {}
impl<T: AsyncReadRent + AsyncWriteRent> DuplexStream for T {}

/// Stream with data codec by ONE protocol
#[derive(Debug, QuickImpl)]
pub struct ONELayer<S: DuplexStream> {
    #[quick_impl(impl Deref, impl DerefMut)]
    inner: S,
    meta: Meta,
}
impl<S: DuplexStream> ONELayer<S> {
    pub async fn from_duplex(mut raw: S) -> Result<Self, Error> {
        let len = raw.read_u8().await?;
        let buf = BytesMut::with_capacity(len as usize);
        let (result, buf) = raw.read_exact(buf).await;
        result?; // TODO: ugly parser code, arguing for a api change at https://github.com/bytedance/monoio/issues/307
        let meta = protocol::one::decode(&buf)?;
        Ok(Self { inner: raw, meta })
    }
    pub async fn connect(&self) -> Result<TcpStream, io::Error> {
        self.meta.cmd.dst.connect().await
    }
}

#[derive(Debug, QuickImpl)]
pub struct Aes256Layer<S: DuplexStream> {
    inner: S,
    cipher: protocol::aes::Codec256,
    buf: BytesMut,
    pos: usize,
}

impl<S: DuplexStream> Aes256Layer<S> {
    pub fn from_duplex(raw: S, key: [u8; 32]) -> Result<Self, CodecError> {
        let cipher = protocol::aes::Codec256::new(key);
        Ok(Self {
            inner: raw,
            cipher,
            buf: BytesMut::with_capacity(1024),
            pos: 0,
        })
    }
    fn buf_exhaust(&self) -> bool {
        self.pos >= self.buf.len()
    }
    async fn read_msg(&mut self) -> Result<Bytes, io::Error>{
        let len = self.inner.read_u32().await?;
        let buf = BytesMut::with_capacity(len as usize);
        let (res, buf_) = self.inner.read_exact(buf).await;
        res?;

        Ok(buf_.freeze())
    }

    async fn send_msg(&mut self, msg: Bytes) -> Result<(), io::Error>{
        let mut buf = BytesMut::with_capacity(size_of::<u32>() + msg.len());
        buf.extend_from_slice(&(msg.len() as u32).to_be_bytes());
        buf.extend_from_slice(&msg);

        let (res, _buf) = self.inner.write(buf).await;
        res?;
        self.inner.flush().await?;

        Ok(())
    }
}
impl<S: DuplexStream> AsyncBufRead for Aes256Layer<S> {
    async fn fill_buf<'s>(&'s mut self) -> std::io::Result<&'s [u8]> {
        if self.pos == self.buf.len() {
            let data = BytesMut::with_capacity(32);
            let (res, mut data) = self.inner.read(data).await;
            match res {
                Ok(len) => {
                    if len != 32 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::ConnectionRefused,
                            ProtocolError::ONE(format!(
                                "AES256 cipher receive a incorrect length data :: {len}"
                            )),
                        ));
                    };

                    let array = GenericArray::from_mut_slice(&mut data);
                    self.cipher.decode(array);
                }
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        e,
                    ))
                }
            }
        }
        Ok(self.buf.as_ref())
    }

    fn consume(&mut self, amt: usize) {
        todo!()
    }
}
