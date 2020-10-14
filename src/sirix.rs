//! This module contains the entrypoint struct for interacting with SirixDB

use super::auth::Auth;
use super::HttpClient;
use hyper::{client::HttpConnector, Client};

/// Toplevel interface
#[derive(Debug)]
pub struct Sirix {
    client: HttpClient,
}

impl Sirix {
    pub fn new(
        base_url: &str,
        username: &str,
        password: &str,
        client: Client<HttpConnector>,
    ) -> Self {
        let auth = Auth::new(username, password, base_url, client).unwrap();
        let http_client = HttpClient::new(auth);
        return Self {
            client: http_client,
        };
    }
    pub async fn authenticate(&mut self) {
        self.client.authenticate().await;
    }
}

pub async fn sirix_init(
    base_url: &str,
    username: &str,
    password: &str,
    client: Client<HttpConnector>,
) -> Sirix {
    let mut sirix = Sirix::new(base_url, username, password, client);
    sirix.authenticate().await;
    return sirix;
}
