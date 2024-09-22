pub use adapter::Adapter;
pub use error::{Error, Result};

pub mod adapter;
pub mod common;
pub mod error;
pub mod abs;

fn main() {
    println!("Hello, world!");
}
