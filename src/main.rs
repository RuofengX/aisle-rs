pub use adapter::Adapter;
pub use common::Payload;
pub use error::{Error, Result};

pub mod adapter;
pub mod common;
pub mod error;

fn main() {
    println!("Hello, world!");
}
