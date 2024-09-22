// 另一种理解方式，由抽象出发

use bytes::BytesMut;
use futures::{Sink, Stream};
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

use crate::{
    common::{Request, Response},
    Error, Result,
};

// 处理单个连接
pub struct Nexus<U, D>
where
    U: Stream<Item = Request> + Sink<Response>,
    D: Stream<Item = Response> + Sink<Request>,
{
    up: U,
    down: D,
}

pub struct RequestCodec(LengthDelimitedCodec);
impl Encoder<Request> for RequestCodec {
    type Error = Error;

    fn encode(&mut self, item: Request, dst: &mut BytesMut) -> Result<()> {
        let b = rmp_serde::to_vec(&item)?;
        self.0.encode(b.into(), dst)?;
        Ok(())
    }
}
impl Decoder for RequestCodec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if let Some(b) = self.0.decode(src)? {
            let item = rmp_serde::from_slice(&b)?;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }
}

// 然后设计一种对应nexus.in的结构，作为处理单个个入站连接的处理器
