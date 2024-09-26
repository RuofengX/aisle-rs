use thiserror::Error;

// pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("not implement error << {0}")]
    NotImplement(String),
    #[error("item codec error << {0}")]
    Codec(#[from] CodecError),
    #[error("convert error << {0}")]
    Convert(#[from] ConvertError),
    #[error("protocol error << {0}")]
    Protocol(#[from] ProtocolError),
    #[error("io error << {0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("convert {0} into {1} error")]
    TryInto(String, String),
}

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("item convert error << {0}")]
    Encode(#[from] rmp_serde::encode::Error),
    #[error("item convert error << {0}")]
    Decode(#[from] rmp_serde::decode::Error),
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("ONE protocol error :: {0}")]
    ONE(String),
    #[error("socks protocol error :: {0}")]
    SocksProtocol(String),
}
