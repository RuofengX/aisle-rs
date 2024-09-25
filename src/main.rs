use std::pin::pin;

use common::{
    cmd::{Command, Verb},
    Exchange, Request,
};
pub use error::Error;
use error::ProtocolError;
use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Framed};
pub mod common;
pub mod config;
pub mod error;
pub mod protocol;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let s = TcpListener::bind("0.0.0.0:9090").await?;
    loop {
        let (st, _addr) = s.accept().await?;
        handler(st).await?;
    }
    Ok(())
}

async fn handler(st: TcpStream) -> Result<usize, Error> {
    let mut exchange = pin!(Exchange::from_tcp(st));
    if let Some(fst_req) = exchange.as_mut().next().await {
        let mut rmt_st = match fst_req {
            Ok(fst_req) => {
                if let Request::Cmd(fst_cmd) = fst_req {
                    match fst_cmd.v {
                        Verb::Connect => fst_cmd.dst.connect().await?,
                        _ => {
                            return Err(Error::NotImplement(format!(
                                "only connect is support yet"
                            )))
                        }
                    }
                } else {
                    return Err(ProtocolError::ONE(format!("first request is not cmd")).into());
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        };
        let rmt_exchange = Framed::new(rmt_st, BytesCodec::new());
        exchange()
    } else {
        return Err(error::ProtocolError::ONE(format!("io close too early")).into());
    }

    Ok(0)
}

async fn connect(cmd: Command) -> Result<TcpStream, Error> {
    Ok(cmd.dst.connect().await?)
}

