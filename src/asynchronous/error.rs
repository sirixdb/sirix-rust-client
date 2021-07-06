//! Error type

use hyper::http;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SirixError {
    // #[error("Unable to communicate with SirixDB server")]
    #[error(transparent)]
    ConnectionError(#[from] hyper::Error),
    //#[error("Malformed JSON response")]
    #[error(transparent)]
    FormatError(#[from] serde_json::error::Error),
    // #[error("Could not build HTTP request")]
    #[error(transparent)]
    ProtocolError(#[from] http::Error),
    // #[error("Invalid URI")]
    #[error(transparent)]
    InvalidUri(#[from] http::uri::InvalidUri),
}

pub type SirixResult<T> = std::result::Result<T, SirixError>;
