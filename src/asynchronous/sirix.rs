//! This module contains the entrypoint struct for interacting with SirixDB

use crate::types::{Json, Xml};

use super::super::info;
use super::super::types::{InfoResults, InfoResultsWithResourcesContainer, Query};
use super::client::{Message, SirixResponse};
use super::database::Database;
use super::http::{
    delete_all, global_info, global_info_string, global_info_with_resources,
    global_info_with_resources_string, post_query,
};
use super::SirixResult;
use serde::de::DeserializeOwned;
use hyper::http::uri::{Authority, Scheme, Uri};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

#[derive(Debug, Clone)]
pub struct Sirix {
    /// the scheme with which to access the SirixDB server
    scheme: Scheme,
    /// the authority at which to access the SirixDB server
    authority: Authority,
    /// the message channel for sending HTTP requests
    channel: Sender<Message>,
    /// the channel containing authentication data
    auth_channel: Option<Receiver<Option<info::TokenData>>>,
}

impl Sirix {
    pub fn new(
        base_uri: Uri,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<info::TokenData>>>,
    ) -> Self {
        return Self {
            scheme: base_uri.scheme().unwrap_or(&Scheme::HTTP).clone(),
            authority: base_uri
                .authority()
                .unwrap_or(&Authority::from_static("localhost:9443"))
                .clone(),
            channel: channel,
            auth_channel: auth_channel,
        };
    }

    pub fn json_database(&self, db_name: String) -> Database<Json> {
        Database::<Json>::new(
            db_name,
            self.scheme.clone(),
            self.authority.clone(),
            self.channel.clone(),
            self.auth_channel.clone(),
        )
    }

    pub fn xml_database(&self, db_name: String) -> Database<Xml> {
        Database::<Xml>::new(
            db_name,
            self.scheme.clone(),
            self.authority.clone(),
            self.channel.clone(),
            self.auth_channel.clone(),
        )
    }

    pub async fn info(&self) -> SirixResult<SirixResponse<InfoResults>> {
        self.info_raw().await
    }

    pub async fn info_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                global_info(
                    self.scheme.clone(),
                    self.authority.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                global_info(
                    self.scheme.clone(),
                    self.authority.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn info_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                global_info_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                global_info_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn info_with_resources(
        &self,
    ) -> SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> {
        self.info_with_resources_raw().await
    }

    pub async fn info_with_resources_raw<U: DeserializeOwned>(
        &self,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                global_info_with_resources(
                    self.scheme.clone(),
                    self.authority.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                global_info_with_resources(
                    self.scheme.clone(),
                    self.authority.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn info_with_resources_string(
        &self,
    ) -> SirixResult<SirixResponse<String>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                global_info_with_resources_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                global_info_with_resources_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn delete_all(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                delete_all(
                    self.scheme.clone(),
                    self.authority.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                delete_all(
                    self.scheme.clone(),
                    self.authority.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn query<U: DeserializeOwned>(
        &self,
        query: Query,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                post_query(
                    self.scheme.clone(),
                    self.authority.clone(),
                    query,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                post_query(
                    self.scheme.clone(),
                    self.authority.clone(),
                    query,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }
}
