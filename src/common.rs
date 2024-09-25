use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures::{Sink, SinkExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpStream,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    error::{CodecError, ConvertError, Error},
    protocol::SerdeCodec,
};

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
        pub async fn connect(&self) -> Result<TcpStream, Error> {
            let st = match self {
                Self::Domain(addr) => TcpStream::connect(addr).await?,
                Self::Socket(addr) => TcpStream::connect(addr).await?,
            };
            Ok(st)
        }
    }
}

pub struct Exchange {
    r: FramedRead<OwnedReadHalf, SerdeCodec<Request>>,
    w: FramedWrite<OwnedWriteHalf, SerdeCodec<Response>>,
}
impl Exchange {
    pub fn from_tcp(st: TcpStream) -> Self {
        let (r, w) = st.into_split();
        Self {
            r: FramedRead::new(r, SerdeCodec::new()),
            w: FramedWrite::new(w, SerdeCodec::new()),
        }
    }
}

impl Stream for Exchange {
    type Item = Result<Request, CodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.r.poll_next_unpin(cx)
    }
}
impl Sink<Response> for Exchange {
    type Error = CodecError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.w.poll_ready_unpin(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Response) -> Result<(), Self::Error> {
        self.w.start_send_unpin(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.w.poll_flush_unpin(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.w.poll_close_unpin(cx)
    }
}

async fn exchange<A, B, Ta, Ra, Tb, Rb>(a: &mut A, b: &mut B) -> Result<(), Error>
where
    A: Stream<Item = Ta> + Sink<Ra, Error = Error> + Unpin,
    B: Stream<Item = Tb> + Sink<Rb, Error = Error> + Unpin,
    Ta: TryInto<Rb, Error = ConvertError>,
    Tb: TryInto<Ra, Error = ConvertError>,
{
    loop {
        tokio::select! {
            Some(t_a) = a.next() =>{
                b.send(t_a.try_into()?).await?;
            },
            Some(t_b) = b.next() =>{
                a.send(t_b.try_into()?).await?
            }
            else => break,
        };
    }
    Ok(())
}
