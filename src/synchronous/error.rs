//! Error type

use thiserror::Error;
use ureq;

#[derive(Error, Debug)]
pub enum SirixError {
    // #[error("Unable to communicate with SirixDB server")]
    #[error(transparent)]
    ConnectionError(#[from] ureq::Error),
    //#[error("Malformed JSON response")]
    #[error(transparent)]
    FormatError(#[from] serde_json::error::Error),
    // #[error("Could not build HTTP request")]
}

pub type SirixResult<T> = std::result::Result<T, SirixError>;
