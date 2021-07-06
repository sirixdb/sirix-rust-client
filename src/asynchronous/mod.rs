pub mod auth;
pub mod client;
pub mod database;
pub mod error;
pub mod http;
pub mod resource;
pub mod sirix;

pub use error::SirixResult;

#[cfg(test)]
mod tests {}
