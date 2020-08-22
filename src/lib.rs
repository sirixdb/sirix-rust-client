//! The `sirix-rust-client` allows Rust applications to interact with SirixDB.

mod auth;
mod constants;
mod http;
mod info;
mod resource;
mod sirix;
mod types;

pub use http::HttpClient;
pub use sirix::Sirix;

#[cfg(test)]
mod tests {
}
