//! This module handles authentication with the SirixDB server

use super::error::SirixError;
use super::info::{TokenData, TokenPostData};
use super::Result as SirixResult;
use bytes::buf::BufExt;
use hyper::http::uri::{Authority, PathAndQuery, Scheme};
use hyper::{
    body, client::HttpConnector, header::HeaderValue, Body, Client, HeaderMap, Method, Request, Uri,
};
use log::{error, warn};
use serde::de::DeserializeOwned;
use serde_json::ser::to_string;

/// The Auth struct is used to handle server authentication
#[derive(Debug)]
pub struct Auth {
    /// A Hyper client to resolve requests to responses.
    client: Client<HttpConnector>,
    /// The scheme for this application
    scheme: Scheme,
    /// The authority for this application
    authority: Authority,
    /// The username for this application
    username: String,
    /// The password for this application
    password: String,
    /// The token data received from the server
    token_data: Option<TokenData>,
}

impl Auth {
    pub fn new(
        username: &str,
        password: &str,
        base_url: &str,
        client: Client<HttpConnector>,
    ) -> SirixResult<Self> {
        // First, validate that passed URL is valid
        // If it's not, we'll kick up an error
        let parsed = base_url.parse::<Uri>()?;
        // If the above line didn't cause an error, we can build the struct
        let authority = parsed.authority().unwrap();
        let scheme = parsed.scheme().unwrap();

        Ok(Self {
            client,
            scheme: scheme.to_owned(),
            authority: authority.to_owned(),
            username: username.to_string(),
            password: password.to_string(),
            token_data: None,
        })
    }

    async fn request_impl<T: DeserializeOwned>(
        &self,
        path_and_query: PathAndQuery,
        method: Method,
        body: Body,
        headers: HeaderMap,
    ) -> SirixResult<T> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(path_and_query)
            .build()
            .unwrap();
        // create request
        let mut request_builder = Request::builder().uri(uri).method(method);
        for header in headers {
            request_builder = request_builder.header(header.0.unwrap(), header.1);
        }
        let request = request_builder.body(body).unwrap();
        // Perform request
        let response = self.client.request(request).await.unwrap();
        // Aggregate body
        let body = body::aggregate(response).await?;

        let parsed_json_response: Result<T, _> = serde_json::from_reader(body.reader());

        match parsed_json_response {
            Ok(response) => Ok(response),
            Err(err) => Err(SirixError::FormatError(err)),
        }
    }

    pub async fn authenticate(&mut self) -> () {
        let mut header_map = HeaderMap::new();
        header_map.append("content-type", HeaderValue::from_static("application/json"));
        let response: std::result::Result<TokenData, SirixError> = self
            .request_impl::<TokenData>(
                PathAndQuery::from_static("/token"),
                Method::POST,
                Body::from(
                    to_string(&TokenPostData {
                        username: self.username.clone(),
                        password: self.password.clone(),
                        grant_type: "password".to_string(),
                    })
                    .unwrap(),
                ),
                header_map,
            )
            .await;
        match response {
            Ok(result) => self.token_data = Some(result),
            Err(err) => warn!("failure to retrieve token using credentials: {:#?}", err),
        }
    }

    async fn refresh(&mut self) -> () {
        let mut header_map = HeaderMap::new();
        header_map.append("content-type", HeaderValue::from_static("application/json"));
        let refresh_json = format!(
            r#"{{"refresh_token":"{}"}}"#,
            self.token_data.as_ref().unwrap().refresh_token.clone()
        );
        let response: std::result::Result<TokenData, SirixError> = self
            .request_impl::<TokenData>(
                PathAndQuery::from_static("/token"),
                Method::POST,
                Body::from(refresh_json),
                header_map,
            )
            .await;
        match response {
            Ok(result) => self.token_data = Some(result),
            Err(err) => warn!("failure to refresh token using refresh_token: {:#?}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::test_mocks;
    use super::*;
    use mockito;
    #[tokio::test]
    async fn test_authenticate() {
        // setup mocks
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        // setup auth struct
        let client = hyper::Client::new();
        let mut auth = Auth::new("admin", "admin", url, client).unwrap();
        // test authenticate
        auth.authenticate().await;
        assert_eq!(auth.token_data.unwrap(), test_mocks::get_token_data());
    }
    #[tokio::test]
    async fn test_refresh() {
        // setup mocks
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let _m2 = test_mocks::mock_refresh();
        // prepare auth struct
        let client = hyper::Client::new();
        let mut auth = Auth::new("admin", "admin", url, client).unwrap();
        auth.authenticate().await;
        // test refresh
        auth.refresh().await;
        assert_ne!(auth.token_data.unwrap(), test_mocks::get_token_data());
    }
}
