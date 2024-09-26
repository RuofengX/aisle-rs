use std::ops::DerefMut;

pub use error::Error;
use monoio::net::TcpListener;
pub mod common;
pub mod config;
pub mod error;
pub mod stream;
pub mod protocol;

#[monoio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let s = TcpListener::bind("0.0.0.0:9090")?;
    loop {
        let (st, _addr) = s.accept().await?;
        let mut crypted = stream::Aes256Layer::from_duplex(st, &[0u8;256])?;
    }
    Ok(())
}
