//! The `sirix-rust-client` allows Rust applications to interact with SirixDB.

mod auth;
mod client;
mod database;
mod error;
mod http;
mod info;
mod mock;
mod resource;
mod sirix;
mod types;

pub use error::SirixResult;

#[cfg(test)]
mod tests {}
