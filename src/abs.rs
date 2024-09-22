// 另一种理解方式，由抽象出发

use std::{
    net::{TcpListener, TcpStream},
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, Stream};

use crate::{common::{Request, Response}, Error, Result};

pub struct Nexus<U, D>
where
    U: Stream<Item = Request> + Sink<Response>,
    D: Stream<Item = Response> + Sink<Request>,
{
    up: U,
    down: D,
}



pub struct AisleIn(TcpStream);
impl AisleIn{
    pub async fn from_tcp(mut stream: TcpStream) -> Result<Self>{
        Ok(Self(stream))

    }
}
impl Stream for AisleIn {
    type Item = Request;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
impl Sink<Response> for AisleIn{
    type Error = Error;
    
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
    
    fn start_send(self: Pin<&mut Self>, item: Response) -> Result<(), Self::Error> {
        todo!()
    }
    
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
    
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

}
