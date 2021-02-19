//! This module handles authentication with the SirixDB server

use super::info::{TokenData, TokenPostData};
use super::{client::request_impl, client::Message, error::SirixError};
use hyper::http::uri::{Authority, PathAndQuery, Scheme};
use hyper::{header::HeaderValue, http::uri::InvalidUri, Body, HeaderMap, Method, Uri};
use log::{error, info};
use serde_json::ser::to_string;
use std::sync::Arc;
use tokio::sync::{mpsc, watch, Notify};
use tokio::time::{sleep, Duration};

async fn authenticate(
    channel: mpsc::Sender<Message>,
    scheme: Scheme,
    authority: Authority,
    username: &str,
    password: &str,
) -> Result<TokenData, SirixError> {
    let mut header_map = HeaderMap::new();
    header_map.append("content-type", HeaderValue::from_static("application/json"));
    let response = request_impl::<TokenData>(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/token"),
        Method::POST,
        header_map,
        Body::from(
            to_string(&TokenPostData {
                username: username.to_string(),
                password: password.to_string(),
                grant_type: "password".to_string(),
            })
            .unwrap(),
        ),
    )
    .await;
    match response {
        Ok(response) => Ok(response.body),
        Err(err) => Err(err),
    }
}

async fn refresh(
    channel: mpsc::Sender<Message>,
    scheme: Scheme,
    authority: Authority,
    token_data: TokenData,
) -> Result<TokenData, SirixError> {
    let mut header_map = HeaderMap::new();
    header_map.append("content-type", HeaderValue::from_static("application/json"));
    let refresh_json = format!(r#"{{"refresh_token":"{}"}}"#, token_data.refresh_token);
    let response = request_impl::<TokenData>(
        channel,
        scheme,
        authority,
        PathAndQuery::from_static("/token"),
        Method::POST,
        header_map,
        Body::from(refresh_json),
    )
    .await;
    match response {
        Ok(response) => Ok(response.body),
        Err(err) => Err(err),
    }
}

async fn begin_authentication_loop(
    channel: mpsc::Sender<Message>,
    scheme: &Scheme,
    authority: &Authority,
    username: &str,
    password: &str,
    loop_kill_receiver: Arc<Notify>,
    response_sender: watch::Sender<Option<TokenData>>,
) -> () {
    let token_data = authenticate(
        channel.clone(),
        scheme.clone(),
        authority.clone(),
        username,
        password,
    )
    .await;
    let token_data = match token_data {
        Ok(token_data) => {
            info!("authentication with credentials successful");
            response_sender.send(Some(token_data.clone())).unwrap();
            Some(token_data)
        }
        Err(_) => {
            error!("authentication with credentials failed");
            None
        }
    };
    let mut token_data = token_data.clone().unwrap();
    loop {
        tokio::select! {
            _ = sleep(Duration::from_secs(token_data.expires_in.clone() - 10)) => (),
            _ = loop_kill_receiver.notified() => {break},
        };
        let refresh_response = refresh(
            channel.clone(),
            scheme.clone(),
            authority.clone(),
            token_data.clone(),
        )
        .await;
        match refresh_response {
            Ok(new_token_data) => {
                info!("authentication with credentials successful");
                token_data = new_token_data;
                response_sender.send(Some(token_data.clone())).unwrap();
            }
            Err(_) => {
                error!("authentication with credentials failed")
            }
        };
    }
}

pub async fn auth(
    username: &str,
    password: &str,
    base_url: &str,
    channel: mpsc::Sender<Message>,
) -> Result<(watch::Receiver<std::option::Option<TokenData>>, Arc<Notify>), InvalidUri> {
    // validate that passed URL is valid
    let base_url = base_url.to_owned();
    base_url.parse::<Uri>()?;

    let username = username.to_owned();
    let password = password.to_owned();

    let (watch_tx, watch_rx) = watch::channel::<Option<TokenData>>(None);
    let kill_switch = Arc::new(Notify::new());
    let kill_switch_receiver = kill_switch.clone();
    tokio::spawn(async move {
        let parsed = base_url.parse::<Uri>().unwrap();
        let authority = parsed.authority().unwrap();
        let scheme = parsed.scheme().unwrap();
        begin_authentication_loop(
            channel,
            scheme,
            authority,
            &username,
            &password,
            kill_switch_receiver,
            watch_tx,
        )
        .await;
    });
    return Ok((watch_rx, kill_switch));
}

#[cfg(test)]
mod tests {
    use super::super::client::spawn_client;
    use super::super::mock::test_mocks;
    use super::*;
    use hyper::http::uri::Uri;
    use mockito;
    #[tokio::test]
    async fn test_authenticate() {
        // setup mocks
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        spawn_client(hyper::Client::new(), receiver);
        // test authenticate
        let parsed = url.parse::<Uri>().unwrap();
        let response = authenticate(
            sender,
            parsed.scheme().unwrap().to_owned(),
            parsed.authority().unwrap().to_owned(),
            "admin",
            "admin",
        )
        .await;
        assert_eq!(response.unwrap(), test_mocks::get_token_data());
    }
    #[tokio::test]
    async fn test_refresh() {
        // setup mocks
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let _m2 = test_mocks::mock_refresh();
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        spawn_client(hyper::Client::new(), receiver);
        let parsed = url.parse::<Uri>().unwrap();
        let response = authenticate(
            sender.clone(),
            parsed.scheme().unwrap().to_owned(),
            parsed.authority().unwrap().to_owned(),
            "admin",
            "admin",
        )
        .await;
        let token_data = response.unwrap();
        assert_eq!(token_data.clone(), test_mocks::get_token_data());
        // test refresh
        let refresh_response = refresh(
            sender,
            parsed.scheme().unwrap().to_owned(),
            parsed.authority().unwrap().to_owned(),
            token_data,
        )
        .await;
        assert_ne!(refresh_response.unwrap(), test_mocks::get_token_data());
    }
    #[tokio::test]
    async fn test_auth_func() {
        // setup mocks
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let _m2 = test_mocks::mock_refresh();
        // setup for auth function
        let (sender, receiver) = tokio::sync::mpsc::channel(32);
        spawn_client(hyper::Client::new(), receiver);
        // initiate auth coroutine
        let (watch_rx, kill_switch) = auth("admin", "admin", url, sender).await.unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(
            *watch_rx.borrow().as_ref().unwrap(),
            test_mocks::get_token_data()
        );
        // advance time to refresh
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(test_mocks::get_token_data().expires_in)).await;
        tokio::time::resume();
        tokio::time::sleep(Duration::from_millis(10)).await;
        // check that auth token was replaced
        assert_ne!(
            *watch_rx.borrow().as_ref().unwrap(),
            test_mocks::get_token_data()
        );
        // shut down auth coroutine
        kill_switch.notify_one();
    }
}
