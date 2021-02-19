//! This module handles the HTTP interface to a running SirixDB server.

use super::client::{request_impl, request_impl_fire_no_response, Message, SirixResponse};
use super::error::SirixResult;
use super::types::*;
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
        PathAndQuery::from_static("/"),
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
    Ok(request_impl_fire_no_response(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/"),
        Method::DELETE,
        header_map,
        Body::empty(),
    )
    .await)
}

/// `PUT /<db_name>`
///
/// Create a new database with name `db_name` and type `db_type`.
pub async fn create_database(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
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
        None => (),
    };
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );

    Ok(request_impl_fire_no_response(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}", db_name)).unwrap(),
        Method::PUT,
        header_map,
        Body::empty(),
    )
    .await)
}

/// `GET /<db_name>`
///
/// Return information about database with name `db_name`.
pub async fn get_database_info(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<DbInfo>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => (),
    };
    header_map.append("accept", HeaderValue::from_static("application/json"));

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}", db_name)).unwrap(),
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
        None => (),
    };

    Ok(request_impl_fire_no_response(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}", db_name)).unwrap(),
        Method::DELETE,
        header_map,
        Body::empty(),
    )
    .await)
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
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<bool>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => (),
    };
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );

    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}", db_name, name)).unwrap(),
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
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => (),
    };
    header_map.append(
        "content-type",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}", db_name, name)).unwrap(),
        Method::PUT,
        header_map,
        Body::from(initial_data),
    )
    .await
}
/// `GET /<db_name>/<name>`
///
/// Read resource with given parameters
pub async fn read_resource<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    params: Vec<(String, String)>,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => (),
    };
    header_map.append(
        "accept",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );
    let params = params
        .iter()
        .map(|param| param.0.to_owned() + "=" + param.1.as_ref())
        .collect::<Vec<String>>()
        .join("&");
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}?{}", db_name, name, params)).unwrap(),
        Method::GET,
        header_map,
        Body::empty(),
    )
    .await
}
/// `GET /<db_name>/<name>/history`
///
/// Get the commits in the history of the resource
pub async fn resource_history<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    authorization: Option<&str>,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    let mut header_map = HeaderMap::new();
    match authorization {
        Some(authorization) => {
            header_map.append(
                "authorization",
                HeaderValue::from_str(authorization).unwrap(),
            );
        }
        None => (),
    };
    header_map.append(
        "accept",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}/history", db_name, name)).unwrap(),
        Method::PUT,
        header_map,
        Body::empty(),
    )
    .await
}
/// `GET /<db_name>/<name>/diff`
///
/// Get diffs for the given revisions
pub async fn diff_resource<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    name: &str,
    params: Vec<(&str, &str)>,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    // TODO automatically serialize diffs
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    let params = params
        .iter()
        .map(|param| param.0.to_owned() + "=" + param.1)
        .collect::<Vec<String>>()
        .join("&");
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}/diff?{}", db_name, name, params)).unwrap(),
        Method::GET,
        header_map,
        Body::empty(),
    )
    .await
}
/// `POST /`
///
/// Submit a global query
pub async fn post_query<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    query: Query,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<T>> {
    // TODO automatically serialize diffs
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/"),
        Method::POST,
        header_map,
        Body::from(serde_json::to_string(&query).unwrap()),
    )
    .await
}
/// `GET /<db_name>/<name>`
///
/// Read resource with given parameters
pub async fn get_etag<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    params: Vec<(&str, &str)>,
    authorization: &str,
    channel: Sender<Message>,
) -> SirixResult<SirixResponse<String>> {
    let mut header_map = HeaderMap::new();
    header_map.append(
        "authorization",
        HeaderValue::from_str(authorization).unwrap(),
    );
    header_map.append(
        "accept",
        HeaderValue::from_str(&db_type.to_string()).unwrap(),
    );
    let params = params
        .iter()
        .map(|param| param.0.to_owned() + "=" + param.1)
        .collect::<Vec<String>>()
        .join("&");
    let response: SirixResult<SirixResponse<T>> = request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!("/{}/{}?{}", db_name, name, params)).unwrap(),
        Method::HEAD,
        header_map,
        Body::empty(),
    )
    .await;
    match response {
        Ok(response) => Ok(SirixResponse {
            status: response.status,
            body: response
                .headers
                .clone()
                .get("etag")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            headers: response.headers,
        }),
        Err(err) => Err(err),
    }
}
/// `POST /<db_name>/<name>`
///
/// Update resource with data according to given parameters
pub async fn update_resource<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    node_id: u128,
    insert: Insert,
    data: String,
    etag: String,
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
    header_map.append("etag", HeaderValue::from_str(&etag).unwrap());
    request_impl(
        channel,
        scheme,
        authority,
        PathAndQuery::from_str(&format!(
            "/{}/{}?nodeId={}&insert={}",
            db_name, name, node_id, insert
        ))
        .unwrap(),
        Method::POST,
        header_map,
        Body::from(data),
    )
    .await
}
/// `POST /<db_name>/<name>`
///
/// Update resource with data according to given parameters
pub async fn resource_delete<T: DeserializeOwned>(
    scheme: Scheme,
    authority: Authority,
    db_name: &str,
    db_type: DbType,
    name: &str,
    node_and_etag: Option<NodeIdAndEtag>,
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
    match node_and_etag {
        // delete node in resource
        Some(data) => {
            header_map.append("etag", HeaderValue::from_str(&data.etag).unwrap());
            Ok(request_impl_fire_no_response(
                channel,
                scheme,
                authority,
                PathAndQuery::from_str(&format!("/{}/{}?nodeId={}", db_name, name, data.node_id))
                    .unwrap(),
                Method::DELETE,
                header_map,
                Body::empty(),
            )
            .await)
        }
        // delete the resource itself
        None => Ok(request_impl_fire_no_response(
            channel,
            scheme,
            authority,
            PathAndQuery::from_str(&format!("/{}/{}", db_name, name)).unwrap(),
            Method::DELETE,
            header_map,
            Body::empty(),
        )
        .await),
    }
}
