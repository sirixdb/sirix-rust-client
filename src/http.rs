//! This module handles the HTTP interface to a running SirixDB server.

use super::client::{request_impl, Message, SirixResponse};
use super::error::SirixResult;
use super::{constants::*, types::*};
// use bytes::Bytes;
// use futures_core::Stream;
use hyper::http::uri::{Authority, PathAndQuery, Scheme};
use hyper::{header::HeaderValue, Body, HeaderMap, Method};
use serde::de::DeserializeOwned;
// use std::error::Error;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;

/// Wrapper for the asynchronous HTTP client, to call SirixDB endpoints.

/// `GET /`
pub async fn global_info(
    scheme: Scheme,
    authority: Authority,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<InfoResults>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => {}
    };
    header_map.append("accept", HeaderValue::from_static("application/json"));

    // Perform request
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/?withResources=true"),
        Method::GET,
        header_map,
        Body::empty(),
    )
    .await
}
/// `GET /?withResources=true`
pub async fn global_info_with_resources(
    scheme: Scheme,
    authority: Authority,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<InfoResultsWithResources>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => {}
    };
    header_map.append("accept", HeaderValue::from_static("application/json"));

    // Perform request
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/?withResources=true"),
        Method::GET,
        header_map,
        Body::empty(),
    )
    .await
}

/// `DELETE /`
///
/// Careful - will delete all databases and associated resources.
pub async fn delete_all(
    scheme: Scheme,
    authority: Authority,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<()>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => {}
    };

    // perform request
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/"),
        Method::DELETE,
        header_map,
        Body::empty(),
    )
    .await
}

/// `PUT /<db_name>`
///
/// Create a new database with name `db_name` and type `db_type`.
pub async fn create_database(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<()>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&("/".to_owned() + db_name)).unwrap(),
        Method::PUT,
        header_map,
        Body::empty(),
    )
    .await
}

/// `GET /<db_name>`
///
/// Return information about database with name `db_name`.
pub async fn get_database_info(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<DbInfo>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    header_map.append("accept", HeaderValue::from_static("application/json"));

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&("/".to_owned() + db_name)).unwrap(),
        Method::GET,
        header_map,
        Body::empty(),
    )
    .await
}

/// `DELETE /<db_name>`
///
/// Delete database with name `db_name`.
pub async fn delete_database(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<()>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&("/".to_owned() + db_name)).unwrap(),
        Method::DELETE,
        header_map,
        Body::empty(),
    )
    .await
}
/// `HEAD /<db_name>/<name>`
///
/// Head request to resource, to determine if resource exists
pub async fn resource_exists(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<bool>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&("/".to_owned() + db_name + name)).unwrap(),
        Method::DELETE,
        header_map,
        Body::empty(),
    )
    .await
}
/// `PUT /<db_name>/<name>`
///
/// Put request to create resource, overwrites if it already exists
pub async fn create_resource<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    initial_data: String,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&("/".to_owned() + db_name + name)).unwrap(),
        Method::PUT,
        header_map,
        Body::from(initial_data),
    )
    .await
}
