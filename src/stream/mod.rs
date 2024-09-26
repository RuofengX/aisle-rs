use std::future::Future;
use std::io;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockSizeUser, KeyInit};
use aes::Aes256;
use bytes::BytesMut;
use monoio::buf::IoBufMut;
use monoio::io::{AsyncReadRent, AsyncReadRentExt, AsyncWriteRent, BufReader};
use monoio::net::TcpStream;
use monoio::BufResult;
use quick_impl::QuickImpl;

use crate::common::Meta;
use crate::error::CodecError;
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
    cipher: protocol::aes::Codec<Aes256>,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

impl<S: DuplexStream> Aes256Layer<S> {
    pub fn from_duplex(raw: S, key: &[u8]) -> Result<Self, CodecError> {
        let cipher = protocol::aes::Codec::new(
            Aes256::new_from_slice(&key).map_err(|_e| CodecError::KeyLength(key.len()))?,
        );
        Ok(Self { inner: raw, cipher })
    }
}