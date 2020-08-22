//! Error type

use hyper::http;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SirixError {
    #[error("Unable to communicate with SirixDB server")]
    ConnectionError(#[from] hyper::error::Error),
    #[error("Malformed JSON response")]
    FormatError(#[from] serde_json::error::Error),
    #[error("Could not build HTTP request")]
    ProtocolError(#[from] http::Error),
    #[error("Invalid URI")]
    InvalidUri(#[from] http::uri::InvalidUri),
}

pub type Result<T> = std::result::Result<T, SirixError>;
