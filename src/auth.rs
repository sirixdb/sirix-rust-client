//! This module handles authentication with the SirixDB server

// TODO: Maybe std::borrow::Cow<'a, str>?

use super::http::HttpClient;

/// The Auth struct is used to handle server authentication
#[derive(Debug)]
pub struct Auth {
    /// Reference to the client
    client: HttpClient,
    /// The username for this application
    username: String,
    /// The password for this application
    password: String,
}

impl Auth {
    pub fn new(username: &str, password: &str, client: HttpClient) -> Self {
        Self {
            client,
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
