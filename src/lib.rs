//! The `sirix-rust-client` allows Rust applications to interact with SirixDB.

mod auth;
mod constants;
mod client;
mod error;
mod http;
mod info;
mod mock;
mod resource;
mod sirix;
mod types;

pub use error::SirixResult;
//pub use http::HttpClient;
//pub use sirix::sirix_init;
//pub use sirix::Sirix;

#[cfg(test)]
mod tests {}
