//! This module handles authentication with the SirixDB server

// TODO: Maybe std::borrow::Cow<'a, str>?

use super::http::HttpClient;

/// The Auth struct is used to handle server authentication
#[derive(Debug)]
pub struct Auth {
    /// The sync or async client
    client: HttpClient,
    /// The username for this application
    username: String,
    /// The password for this application
    password: String,
    /// Whether we need to check refresh?
    // FIXME: I don't know what this is for yet, write a real docstring when you do
    refresh_check: bool,
}

impl Auth {
    pub fn new(username: &str, password: &str, client: HttpClient) -> Self {
        Self {
            client,
            username: username.to_string(),
            password: password.to_string(),
            refresh_check: true,
        }
    }
}
