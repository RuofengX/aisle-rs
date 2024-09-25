use std::marker::PhantomData;

use bytes::BytesMut;
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

use crate::error::CodecError;

/// A [`tokio_util::codec::Codec`]` implement that combine [`rmp_serde`] and [`LengthDelimitedCodec`]
#[derive(Debug)]
pub struct SerdeCodec<T> {
    inner: LengthDelimitedCodec,
    _t: PhantomData<T>,
}

impl<T> SerdeCodec<T> {
    pub fn new() -> Self {
        SerdeCodec {
            inner: LengthDelimitedCodec::new(),
            _t: PhantomData,
        }
    }
}
impl<T: Serialize + Unpin> Encoder<T> for SerdeCodec<T> {
    type Error = CodecError;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), CodecError> {
        let b = rmp_serde::to_vec(&item)?;
        self.inner.encode(b.into(), dst)?;
        Ok(())
    }
}
impl<T: DeserializeOwned + Unpin> Decoder for SerdeCodec<T> {
    type Item = T;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, CodecError> {
        if let Some(b) = self.inner.decode(src)? {
            let item = rmp_serde::from_slice(&b)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}
