use std::net::{Ipv4Addr, Ipv6Addr};

use bytes::{Buf, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::ToSocketAddrs};
use tokio::net::TcpStream;

use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Connect = 0x00, // Start tcp stream
    Bind = 0x01,    // Serve
    Direct = 0x02,  // UDP
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub cmd: Command,
    pub dst: SocketAddr,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
impl Payload {
    pub fn into_bytes(self) -> Result<Bytes> {
        let b = rmp_serde::to_vec(&self)?;
        let b_len = b.len();

        let mut buf = BytesMut::from(b_len.to_be_bytes().as_slice());
        buf.extend_from_slice(b.as_slice());
        Ok(buf.freeze())
    }

    pub fn from_bytes(bytes: &mut Bytes) -> Result<Option<Self>> {
        if bytes.len() < 8 {
            return Ok(None);
        }

        let mut b_len_buf = [0u8; 8];
        bytes.copy_to_slice(&mut b_len_buf);

        let b_len = usize::from_be_bytes(b_len_buf);
        let b = bytes.copy_to_bytes(b_len);

        let buf = rmp_serde::from_slice(&b)?;
        Ok(Some(buf))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Address {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    Domain(Bytes),
}
impl Address {
    pub async fn from_socks5(stream: &mut TcpStream) -> Result<Self> {
        let atyp = stream.read_u8().await?;
        match atyp {
            0x01 => {
                let addr = Ipv4Addr::from_bits(stream.read_u32().await?);
                Ok(Self::V4(addr))
            }
            0x03 => {
                let addr_len = stream.read_u8().await?;
                let mut buf = BytesMut::with_capacity(addr_len as usize);
                stream.read_exact(&mut buf).await?;
                Ok(Self::Domain(buf.freeze()))
            }
            0x04 => {
                let addr = Ipv6Addr::from_bits(stream.read_u128().await?);
                Ok(Self::V6(addr))
            }
            _ => Err(Error::SocksProtocol(format!("undefined atyp {atyp}"))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketAddr {
    pub addr: Address,
    pub port: u16,
}
impl ToSocketAddrs for SocketAddr{

}
