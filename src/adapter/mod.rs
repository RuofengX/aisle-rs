pub mod socks;


use async_trait::async_trait;
use futures::{Sink, SinkExt, Stream, StreamExt};

use crate::{Error, Result};

#[async_trait]
pub trait Adapter<T1, T2>
where
    T1: TryInto<T2, Error = Error> + Unpin + Send,
    T2: TryInto<T1, Error = Error> + Unpin + Send,
{
    async fn exchange<A, B>(
        &mut self,
        mut a: impl Stream<Item = T1> + Sink<T1, Error = Error> + Unpin + Send,
        mut b: impl Stream<Item = T2> + Sink<T2, Error = Error> + Unpin + Send,
    ) -> Result<()> {
        loop {
            tokio::select! {
                Some(item) = a.next() =>{
                    b.send(item.try_into()?).await?;
                }
                Some(item) = b.next() =>{
                    a.send(item.try_into()?).await?;
                }
                else => break,

            };
        }
        Ok(())
    }
}
