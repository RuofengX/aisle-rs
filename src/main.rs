use std::marker::PhantomData;

use futures::{Sink, SinkExt, Stream, StreamExt};
use thiserror::Error;

pub struct Relay<T, U, I, O>
where
    T: TryInto<U, Error = Error>,
    I: Stream<Item = T> + Unpin,
    O: Sink<U, Error = Error> + Unpin,
{
    from: I,
    to: O,
    _t1: PhantomData<T>,
    _t2: PhantomData<U>,
}

impl<T1, T2, I, O> Relay<T1, T2, I, O>
where
    T1: TryInto<T2, Error = Error>,
    I: Stream<Item = T1> + Unpin,
    O: Sink<T2, Error = Error> + Unpin,
{
    pub fn new(from: I, to: O) -> Self {
        Self {
            from,
            to,
            _t1: PhantomData,
            _t2: PhantomData,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        while let Some(item) = self.from.next().await {
            let item2 = item.try_into()?;
            self.to.send(item2).await?;
        }
        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("relay error")]
    Relay,
    #[error("convert error")]
    Convert,
}

fn main() {
    println!("Hello, world!");
}
