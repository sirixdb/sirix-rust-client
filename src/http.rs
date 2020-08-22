//! This module handles the HTTP interface

use hyper::{client::HttpConnector, Client, Uri};
use super::types::*;

/// Wrapper for the asynchronous HTTP client, to call SirixDB endpoints.
#[derive(Debug)]
pub struct HttpClient {
    /// The base URL for the SirixDB connection.
    pub base_url: Uri,
    /// A Hyper client to resolve requests to responses.
    client: Client<HttpConnector>,
}

impl HttpClient {
    /// Construct a new client.  Expects the URL for the running SirixDB server.
    ///
    /// ```
    /// # use sirix_rust_client::HttpClient;
    /// # use pretty_assertions::assert_eq;
    /// let client = HttpClient::new("http://localhost:9443");
    /// # assert_eq!(&client.base_url.to_string(), "http://localhost:9443/")
    /// ```
    pub fn new(url: &str) -> Self {
        Self {
            base_url: url.parse().expect("Should parse base URI"),
            client: Client::new(),
        }
    }

    /// GET '/'
    ///
    /// If `resources` is `true`, appends query param `withResources=true`.
    async fn global_info(&self) -> Vec<InfoResult> {
        unimplemented!()
    }
}
