use std::future::Future;

use aes::Aes256;
use bytes::BytesMut;
use monoio::buf::{IoBuf, IoBufMut, IoVecBuf, IoVecBufMut};
use monoio::io::{AsyncReadRent, AsyncReadRentExt, AsyncWriteRent};
use monoio::BufResult;
use quick_impl::QuickImpl;

use crate::common::Meta;
use crate::protocol::AES;
use crate::{protocol, Error};

pub trait DuplexStream: AsyncReadRent + AsyncWriteRent {}
impl<T: AsyncReadRent + AsyncWriteRent> DuplexStream for T {}

/// Stream with data codec by ONE protocol
#[derive(Debug,QuickImpl)]
pub struct ONELayer<S: DuplexStream> {
    #[quick_impl(impl AsMut)]
    inner: S,
    meta: Meta,
}
impl<S: DuplexStream> ONELayer<S> {
    pub async fn from_duplex(mut raw: S) -> Result<Self, Error> {
        let len = raw.read_u8().await?;
        let buf = BytesMut::with_capacity(len as usize);
        let (result, buf) = raw.read_exact(buf).await;
        result?; // TODO: ugly parser code, arguing for a api change at https://github.com/bytedance/monoio/issues/307
        let meta = protocol::ONE::decode(&buf)?;
        Ok(Self { inner: raw, meta })
    }
}

pub struct Aes256Layer<S: DuplexStream> {
    inner: S,
    cipher: AES::Codec<Aes256>,
}
