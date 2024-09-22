use bytes::Bytes;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("not supported yet")]
    NotImplement,
    #[error("relay error")]
    Relay,
    #[error("item convert error")]
    Convert,
    #[error("item convert error")]
    Encode(#[from] rmp_serde::encode::Error),
    #[error("item convert error")]
    Decode(#[from] rmp_serde::decode::Error),
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("socks protocol error: {0}")]
    SocksProtocol(String),
}
