//! This module contains the entrypoint struct for interacting with SirixDB

use super::auth::Auth;
use super::http::HttpClient;

/// Toplevel interface
#[derive(Debug)]
pub struct Sirix {
    auth: Auth,
    client: HttpClient,
}
