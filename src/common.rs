use std::{io, net::SocketAddr};

use monoio::net::TcpStream;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub cmd: Command,
}

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
    pub async fn connect(&self) -> Result<TcpStream, io::Error> {
        let st = match self {
            Self::Domain(addr) => TcpStream::connect(addr).await?,
            Self::Socket(addr) => TcpStream::connect(addr).await?,
        };
        Ok(st)
    }
}
