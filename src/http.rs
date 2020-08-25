//! This module handles the HTTP interface to a running SirixDB server.

use super::{constants::*, types::*, Result};
use bytes::buf::BufExt;
use hyper::{
    body, client::HttpConnector, header::HeaderValue, Body, Client, Method, Request, StatusCode,
    Uri,
};
use std::io::Read;

/// Wrapper for the asynchronous HTTP client, to call SirixDB endpoints.
#[derive(Debug)]
pub struct HttpClient {
    /// The base URL for the SirixDB connection.
    pub base_url: String,
    /// A Hyper client to resolve requests to responses.
    client: Client<HttpConnector>,
}

impl HttpClient {
    /// Construct a new client.  Expects the URL for the running SirixDB server.
    ///
    /// ```
    /// # use sirix_rust_client::HttpClient;
    /// # use pretty_assertions::assert_eq;
    /// let client = HttpClient::new("http://localhost:9443/").unwrap();
    /// # assert_eq!(&client.base_url, "http://localhost:9443/")
    /// ```
    pub fn new(url: &str) -> Result<Self> {
        // First, validate that passed URL is valid
        // If it's not, we'll kick up an error, but we're still just storing a string
        let _ = url.parse::<Uri>()?;

        // If the above line didn't cause an error, we can build the struct
        Ok(Self {
            base_url: url.to_string(),
            client: Client::new(),
        })
    }

    /// `GET /`
    ///
    /// If `resources` is `true`, appends query param `withResources=true`.
    pub async fn global_info(&self, resources: bool) -> Result<InfoResults> {
        // FIXME: I feel like there's a better way than just pasting a string
        let uri: Uri = if resources {
            format!("{}?withResources=true", self.base_url).parse()?
        } else {
            self.base_url.parse()?
        };

        // Perform request
        let response = self.client.get(uri).await?;
        // Aggregate body
        let body = body::aggregate(response).await?;
        // Parse json
        let full_list = serde_json::from_reader(body.reader())?;

        Ok(full_list)
    }

    /// `DELETE /`
    ///
    /// Careful - will delete all databases and associated resources.
    pub async fn delete_all(&self) -> Result<()> {
        let req = Request::builder()
            .method(Method::DELETE)
            .uri(&self.base_url)
            .body(Body::default())?;

        // Don't pass the response back up on success.  Just fire it off and return if successful.
        let _ = self.client.request(req).await?;
        Ok(())
    }

    /// `PUT /<db_name>`
    ///
    /// Create a new database with name `db_name`.
    pub async fn create_database(&self, db_name: String, db_type: DbType) -> Result<()> {
        let req = Request::builder()
            .method(Method::PUT)
            .header("content-type", &db_type.to_string())
            .uri(&self.base_url)
            .body(Body::from(db_name))?;

        // Don't pass the response back up on success.  Just fire it off and return if successful.
        let _ = self.client.request(req).await?;
        Ok(())
    }

    /// `GET /<db_name>`
    ///
    /// Return information about database with name `db_name`.
    pub async fn get_database_info(&self, db_name: String) -> Result<DbInfo> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(&self.base_url)
            .body(Body::from(db_name))?;

        // Perform request
        let response = self.client.request(req).await?;
        // Aggregate body
        let body = body::aggregate(response).await?;
        // Parse json
        let db_info = serde_json::from_reader(body.reader())?;

        Ok(db_info)
    }

    /// `DELETE /<db_name>`
    ///
    /// Delete database with name `db_name`.
    pub async fn delete_database(&self, db_name: String) -> Result<()> {
        let req = Request::builder()
            .method(Method::DELETE)
            .uri(&self.base_url)
            .body(Body::from(db_name))?;

        // Don't pass the response back up on success.  Just fire it off and return if successful.
        let _ = self.client.request(req).await?;
        Ok(())
    }

    /// `HEAD /<db_name>/<resource>`
    ///
    /// Verify the given resource exists in given database.
    pub async fn resource_exists(
        &self,
        db_name: &str,
        db_type: DbType,
        resource: &str,
    ) -> Result<bool> {
        let req = Request::builder()
            .method(Method::HEAD)
            .header("content-type", &db_type.to_string())
            .uri(self.resource_url(db_name, resource))
            .body(Body::empty())?;

        let resp = self.client.request(req).await?;
        Ok(resp.status() == StatusCode::OK)
    }

    /// `PUT /<db_name>/<resource>`
    ///
    /// Create a resource with given name in given database.
    pub async fn create_resource(
        &self,
        db_name: &str,
        db_type: DbType,
        resource: &str,
    ) -> Result<String> {
        let req = Request::builder()
            .method(Method::HEAD)
            .header("content-type", &db_type.to_string())
            .uri(self.resource_url(db_name, resource))
            .body(Body::empty())?;

        // Aggregate body
        let resp = self.client.request(req).await?;
        let body = body::aggregate(resp).await?;

        // Read to a String
        let mut text = String::new();
        body.reader().read_to_string(&mut text)?;

        Ok(text)
    }

    /// Helper function to build resource URLs
    #[inline]
    fn resource_url(&self, db_name: &str, resource: &str) -> Uri {
        format!("{}{}/{}", &self.base_url, db_name, resource)
            .parse()
            .unwrap()
    }
}
