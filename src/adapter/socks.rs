use core::str;
use std::ops::Deref;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use bytes::{Bytes, BytesMut};

use crate::common::{Address, Command, SocketAddr};
use crate::error::Result;
use crate::Error;

pub struct SocksServer {
    upstream: TcpListener,
    guards: Vec<Guard>,
}

impl SocksServer {
    pub const VERSION: u8 = 0x05;
    pub async fn serve(&mut self) -> Result<()> {
        loop {
            match self.upstream.accept().await {
                Ok((stream, _peer)) => {
                    self.init(stream).await?;
                }
                Err(e) => {
                    println!("{e}");
                }
            }
        }
    }
    async fn init(&self, mut stream: TcpStream) -> Result<()> {
        verify_socks_version(Self::VERSION, &mut stream).await?;

        let methods = stream.read_u8().await?;
        let mut methods_buf = BytesMut::with_capacity(methods as usize);
        stream.read_exact(&mut methods_buf).await?;

        if let Some(&matched_guard) = self
            .guards
            .iter()
            .find(|&&g| methods_buf.contains(g.deref()))
        {
            stream.write_u8(0x05).await?;
            stream.write_u8(*matched_guard).await?;

            matched_guard.verify(&mut stream).await?;
            let exchange = Exchange::new(&mut stream).await?;
            todo!()
        } else {
            stream.write_u8(0x05).await?;
            stream.write_u8(*Guard::NoAcceptable).await?;
            return Err(Error::SocksProtocol(format!("no auth method available")));
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Guard {
    NoAuth,
    GSSAPI,
    Password {
        uname: &'static str,
        passwd: &'static str,
    },
    NoAcceptable,
}

impl Guard {
    pub async fn verify(&self, stream: &mut TcpStream) -> Result<()> {
        match self {
            Self::NoAuth => Ok(()),
            Self::GSSAPI => Err(Error::NotImplement),
            Self::Password {
                uname: guard_uname,
                passwd: guard_passwd,
            } => {
                verify_socks_auth_version(0x01, stream).await?;
                let ulen = stream.read_u8().await?;
                let mut uname = BytesMut::with_capacity(ulen as usize);
                stream.read_exact(&mut uname).await?;

                let plen = stream.read_u8().await?;
                let mut passwd = BytesMut::with_capacity(plen as usize);
                stream.read_exact(&mut passwd).await?;

                if uname == guard_uname.as_bytes() && passwd == guard_passwd.as_bytes() {
                    stream.write_u8(0x01).await?;
                    stream.write_u8(0x00).await?;
                    Ok(())
                } else {
                    stream.write_u8(0x01).await?;
                    stream.write_u8(0xff).await?;
                    return Err(Error::SocksProtocol(format!("wrong password")));
                }
            }
            Self::NoAcceptable => unreachable!(),
        }
    }
}
impl Deref for Guard {
    type Target = u8;

    fn deref(&self) -> &'static Self::Target {
        match self {
            Self::NoAuth => &0x00,
            Self::GSSAPI => &0x01,
            Self::Password {
                uname: _,
                passwd: _,
            } => &0x02,
            Self::NoAcceptable => &0xff,
        }
    }
}

pub struct Exchange<'s> {
    cmd: Command,
    dst: SocketAddr,
    upstream: &'s TcpStream,
}
impl<'s> Exchange<'s> {
    pub async fn new(stream: &'s mut TcpStream) -> Result<Self> {
        verify_socks_version(0x05, stream).await?;

        let cmd = stream.read_u8().await?;
        let cmd = match cmd {
            0x01 => Command::Connect,
            _ => return Err(Error::SocksProtocol(format!("unsupported command {cmd}"))),
        };
        let rsv = stream.read_u8().await?;
        if rsv != 0x00 {
            return Err(Error::SocksProtocol(format!("wrong rsv byte {rsv}")));
        }

        let dst = SocketAddr {
            addr: Address::from_socks5(stream).await?,
            port: stream.read_u16().await?,
        };

        Ok(Self {
            cmd,
            dst,
            upstream: stream,
        })
    }

    // Start the stream exchange
    pub async fn init(&mut self) -> Result<usize>{
        match self.cmd{
            Command::Connect=>{
                let remote = TcpStream::connect(addr)
            }
            _ => Err(Error::NotImplement)
        }
    }
}

async fn verify_socks_version(allowed_version: u8, stream: &mut TcpStream) -> Result<()> {
    let ver = stream.read_u8().await?;
    if ver != allowed_version {
        return Err(Error::SocksProtocol(format!(
            "wrong socks version: {0}",
            ver
        )));
    }
    Ok(())
}
async fn verify_socks_auth_version(allowed_version: u8, stream: &mut TcpStream) -> Result<()> {
    let ver = stream.read_u8().await?;
    if ver != allowed_version {
        return Err(Error::SocksProtocol(format!(
            "wrong socks auth version: {0}",
            ver
        )));
    }
    Ok(())
}
