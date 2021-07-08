//! The `sirix-rust-client` allows Rust applications to interact with SirixDB.

#[cfg(feature = "async")]
pub mod asynchronous;

#[cfg(feature = "sync")]
pub mod synchronous;

pub mod info;
pub mod mock;
pub mod types;
mod utils;

#[cfg(test)]
mod tests {}
