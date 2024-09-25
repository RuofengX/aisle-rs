use std::net::SocketAddr;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Cmd(cmd::Command),
    Data(Bytes),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response(Bytes);

pub mod cmd {
    use super::*;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Command {
        pub v: Verb,
        pub dst: Destination,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum Verb {
        Connect,
        Bind,
        UDP,
    }
    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub enum Destination {
        Domain(String),
        Socket(SocketAddr),
    }

    impl Destination {
        pub async fn connect(&self) -> Result<TcpStream, AppError> {
            let st = match self {
                Self::Domain(addr) => TcpStream::connect(addr).await?,
                Self::Socket(addr) => TcpStream::connect(addr).await?,
            };
            Ok(st)
        }
    }
}
