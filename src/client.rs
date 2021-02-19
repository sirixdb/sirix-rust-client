use super::error::SirixError;
use super::SirixResult;
use hyper::http::status::StatusCode;
use hyper::http::uri::{Authority, PathAndQuery, Scheme};
use hyper::{
    body, body::Buf, header::HeaderValue, Body, Error, HeaderMap, Method, Request, Response, Uri,
};
use hyper::{client::HttpConnector, Client};
use serde::de::DeserializeOwned;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub type ResultResponse = Result<Response<Body>, Error>;

pub struct SirixResponse<T> {
    pub status: StatusCode,
    pub headers: HeaderMap<HeaderValue>,
    pub body: T,
}

#[derive(Debug)]
pub struct Message {
    pub request: Request<Body>,
    pub responder: oneshot::Sender<ResultResponse>,
}

pub fn spawn_client(
    client: Client<HttpConnector>,
    mut channel: Receiver<Message>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let message = select! {
                // if the channel sent a message, get the message and send the HTTP request
                Some(message) = channel.recv() => message,
                // if the channel closed, break the loop, and the task will terminate
                else => {break},
            };
            // make the HTTP request
            let response = client.request(message.request).await;
            // return the response (or error) to the caller
            message.responder.send(response).unwrap();
        }
    })
}

pub async fn request_impl<T: DeserializeOwned>(
    channel: Sender<Message>,
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> SirixResult<SirixResponse<T>> {
    let uri = Uri::builder()
        .scheme(scheme)
        .authority(authority)
        .path_and_query(path_and_query)
        .build()
        .unwrap();
    // create request
    let mut request_builder = Request::builder().uri(uri).method(method);
    for header in headers {
        request_builder = request_builder.header(header.0.unwrap(), header.1);
    }
    let request = request_builder.body(body).unwrap();
    // create response channel
    let (tx, rx) = oneshot::channel::<ResultResponse>();
    // Perform request
    let _ = channel
        .send(Message {
            request: request,
            responder: tx,
        })
        .await;
    let response = rx.await.unwrap().unwrap();
    let status = response.status().clone();
    let headers = response.headers().clone();
    // Aggregate body
    let body = body::aggregate(response).await?;

    let parsed_json_response: Result<T, _> = serde_json::from_reader(body.reader());

    match parsed_json_response {
        Ok(parsed) => Ok(SirixResponse {
            headers: headers.to_owned(),
            status: status,
            body: parsed,
        }),
        Err(err) => Err(SirixError::FormatError(err)),
    }
}

pub async fn request_impl_fire_no_response(
    channel: Sender<Message>,
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> SirixResponse<()> {
    let uri = Uri::builder()
        .scheme(scheme)
        .authority(authority)
        .path_and_query(path_and_query)
        .build()
        .unwrap();
    // create request
    let mut request_builder = Request::builder().uri(uri).method(method);
    for header in headers {
        request_builder = request_builder.header(header.0.unwrap(), header.1);
    }
    let request = request_builder.body(body).unwrap();
    // create response channel
    let (tx, rx) = oneshot::channel::<ResultResponse>();
    // Perform request
    let _ = channel
        .send(Message {
            request: request,
            responder: tx,
        })
        .await;
    let response = rx.await.unwrap().unwrap();
    let status = response.status().clone();
    let headers = response.headers().clone();
    SirixResponse {
        headers: headers.to_owned(),
        status: status,
        body: (),
    }
}
