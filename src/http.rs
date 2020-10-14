//! This module handles the HTTP interface to a running SirixDB server.

use super::{auth::Auth, constants::*, types::*, Result};
use bytes::buf::BufExt;
use hyper::{body, header::HeaderValue, Body, Method, Request, Uri};

/// Wrapper for the asynchronous HTTP client, to call SirixDB endpoints.
#[derive(Debug)]
pub struct HttpClient {
    auth: Auth,
}

impl HttpClient {
    pub fn new(auth: Auth) -> Self {
        Self { auth: auth }
    }
    pub async fn authenticate(&mut self) {
        self.auth.authenticate().await;
    }
/*
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
    }*/
}
