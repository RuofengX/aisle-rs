use std::{
    marker::PhantomData,
    pin::{pin, Pin},
    task::{Context, Poll},
};

use common::{
    cmd::{Command, Verb},
    Request, Response,
};
pub use error::AppError;
use error::{CodecError, ProtocolError};
use futures::{Sink, SinkExt, Stream, StreamExt};
use protocol::SerdeCodec;
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio_util::codec::{Framed, FramedParts, FramedRead, FramedWrite};

pub mod common;
pub mod config;
pub mod error;
pub mod protocol;

pub struct Frame {
    r: FramedRead<OwnedReadHalf, SerdeCodec<Request>>,
    w: FramedWrite<OwnedWriteHalf, SerdeCodec<Response>>,
}
impl Frame {
    pub fn from_tcp(st: TcpStream) -> Self {
        let (r, w) = st.into_split();
        Self {
            r: FramedRead::new(r, SerdeCodec::new()),
            w: FramedWrite::new(w, SerdeCodec::new()),
        }
    }
}

impl Stream for Frame {
    type Item = Result<Request, CodecError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.r.poll_next_unpin(cx)
    }
}
impl Sink<Response> for Frame {
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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Hello, world!");
    let s = TcpListener::bind("0.0.0.0:9090").await?;
    loop {
        let (st, _addr) = s.accept().await?;
        handler(st).await?;
    }
    Ok(())
}

async fn handler(st: TcpStream) -> Result<usize, AppError> {
    let mut f = pin!(Frame::from_tcp(st));
    if let Some(fst_req) = f.as_mut().next().await {
        match fst_req {
            Ok(fst_req) => {
                if let Request::Cmd(fst_cmd) = fst_req {
                    match fst_cmd.v {
                        Verb::Bind => todo!(),
                        _ => {
                            return Err(AppError::NotImplement(format!("only bind is support yet")))
                        }
                    }
                } else {
                    return Err(ProtocolError::ONE(format!("first request is not cmd")).into());
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    } else {
        return Err(error::ProtocolError::ONE(format!("io close too early")).into());
    }

    Ok(0)
}
