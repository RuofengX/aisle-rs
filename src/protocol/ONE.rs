
use serde::{de::DeserializeOwned, Serialize};

use crate::error::CodecError;

pub fn decode<T: DeserializeOwned>(buf: &[u8]) -> Result<T, CodecError> {
    rmp_serde::from_slice(buf).map_err(|e| CodecError::Decode(e))
}

pub fn encode<T: Serialize>(src: T) -> Result<Vec<u8>, CodecError> {
    rmp_serde::to_vec(&src).map_err(|e| CodecError::Encode(e))
}
