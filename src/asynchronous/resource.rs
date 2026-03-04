//! Working with a Sirix resource.

use super::super::info::TokenData;
use super::super::types::{
    DbType, DiffArgs, History, Insert, Json, MetaNode, MetadataType, NodeIdAndEtag, ReadArgs,
    SingleRevision, Xml,
};
use super::super::utils::build_read_params;
use super::client::{Message, SirixResponse};
use super::http::{
    create_resource, create_resource_string, diff_resource, get_etag, read_resource,
    read_resource_string, resource_delete, resource_exists, resource_history,
    resource_history_string, update_resource,
};
use super::SirixResult;
use hyper::http::uri::{Authority, Scheme};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

///  Struct for manipulating a resource
#[derive(Debug, Clone)]
pub struct Resource<T> {
    _t: T,
    /// The name of the database this resource belongs to.
    pub db_name: String,
    /// The type of that database.
    pub db_type: DbType,
    /// The name of the resource being accessed, or created if it does not yet exist.
    pub resource_name: String,
    /// the scheme with which to access the SirixDB server
    scheme: Scheme,
    /// the authority at which to access the SirixDB server
    authority: Authority,
    /// the message channel for sending HTTP requests
    channel: Sender<Message>,
    /// the channel containing authentication data
    auth_channel: Option<Receiver<Option<TokenData>>>,
}

impl Resource<Json> {
    pub fn new(
        db_name: String,
        resource_name: String,
        scheme: Scheme,
        authority: Authority,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<TokenData>>>,
    ) -> Self {
        Self {
            _t: Json,
            db_name: db_name,
            db_type: DbType::Json(Json),
            resource_name: resource_name,
            scheme: scheme,
            authority: authority,
            channel: channel,
            auth_channel: auth_channel,
        }
    }
}

impl Resource<Xml> {
    pub fn new(
        db_name: String,
        resource_name: String,
        scheme: Scheme,
        authority: Authority,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<TokenData>>>,
    ) -> Self {
        Self {
            _t: Xml,
            db_name,
            db_type: DbType::XML(Xml),
            resource_name,
            scheme,
            authority,
            channel,
            auth_channel,
        }
    }
}

impl<T> Resource<T> {
    pub async fn create_string(&self, initial_data: String) -> SirixResult<SirixResponse<String>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                create_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    initial_data,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                create_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    initial_data,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn create_raw<U: DeserializeOwned>(
        &self,
        initial_data: String,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                create_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    initial_data,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                create_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    initial_data,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn create(
        &self,
        initial_data: String,
    ) -> SirixResult<SirixResponse<serde_json::Value>> {
        self.create_raw(initial_data).await
    }

    pub async fn exists(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                resource_exists(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                resource_exists(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn etag(&self, node_id: u128) -> SirixResult<SirixResponse<String>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                get_etag(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                get_etag(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn read<U: DeserializeOwned>(
        &self,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<U>> {
        let params = build_read_params(read_args);
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                read_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                read_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn read_string(
        &self,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<String>> {
        let params = build_read_params(read_args);
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                read_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                read_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn read_with_metadata_raw<U: DeserializeOwned>(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<U>> {
        let mut params = build_read_params(read_args);
        params.push(("withMetadata".to_owned(), meta_type.to_string()));
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                read_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                read_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn read_with_metadata(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<MetaNode>> {
        self.read_with_metadata_raw(meta_type, read_args).await
    }

    pub async fn read_with_metadata_string(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<String>> {
        let mut params = build_read_params(read_args);
        params.push(("withMetadata".to_owned(), meta_type.to_string()));
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                read_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                read_resource_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn history_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                resource_history(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                resource_history(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn history(&self) -> SirixResult<SirixResponse<History>> {
        self.history_raw().await
    }

    pub async fn history_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                resource_history_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                resource_history_string(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn diff(&self, args: DiffArgs) -> SirixResult<SirixResponse<()>> {
        let mut params: Vec<(String, String)> = Vec::new();
        match args.node_id {
            Some(node_id) => params.push(("startNodeKey".to_owned(), node_id.to_string())),
            None => (),
        }
        match args.max_depth {
            Some(max_depth) => params.push(("maxDepth".to_owned(), max_depth.to_string())),
            None => (),
        }
        match args.first_revision {
            SingleRevision::Timestamp(revision) => {
                params.push(("first-revision".to_owned(), revision))
            }
            SingleRevision::Number(revision) => {
                params.push(("first-revision".to_owned(), revision.to_string()))
            }
        }
        match args.second_revision {
            SingleRevision::Timestamp(revision) => {
                params.push(("second-revision".to_owned(), revision))
            }
            SingleRevision::Number(revision) => {
                params.push(("second-revision".to_owned(), revision.to_string()))
            }
        };
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                diff_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    &self.resource_name,
                    params,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                diff_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    &self.resource_name,
                    params,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn delete(
        &self,
        node_and_etag: Option<NodeIdAndEtag>,
    ) -> SirixResult<SirixResponse<()>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                resource_delete(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_and_etag,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                resource_delete(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_and_etag,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn update<U: DeserializeOwned>(
        &self,
        node_id: u128,
        insert: Insert,
        data: String,
        etag: String,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                update_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                    insert,
                    data,
                    etag,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                update_resource(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                    insert,
                    data,
                    etag,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }
}
