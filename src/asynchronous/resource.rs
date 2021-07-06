//! Working with a Sirix resource.

use super::super::info::TokenData;
use super::super::types::{
    DbType, DiffArgs, History, Json, MetaNode, MetadataType, NodeIdAndEtag, ReadArgs, RevisionArg,
    SingleRevision, TwoRevisions, XML,
};
use super::client::{Message, SirixResponse};
use super::http::{
    create_resource, diff_resource, get_etag, read_resource, resource_delete, resource_exists,
    resource_history,
};
use super::SirixResult;
use hyper::http::uri::{Authority, Scheme};
use serde::de::DeserializeOwned;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

///  Struct for manipulating a resource
#[derive(Debug)]
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

    pub async fn history(&self) -> SirixResult<SirixResponse<History>> {
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

    pub async fn read<T: DeserializeOwned>(
        &self,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<T>> {
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

    pub async fn read_with_metadata(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<MetaNode>> {
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

    // TODO fix return type
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
        }
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

    // TODO
    // update
    // query
}

impl Resource<XML> {
    pub fn new(
        db_name: String,
        resource_name: String,
        scheme: Scheme,
        authority: Authority,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<TokenData>>>,
    ) -> Self {
        Self {
            _t: XML,
            db_name: db_name,
            db_type: DbType::XML(XML),
            resource_name: resource_name,
            scheme: scheme,
            authority: authority,
            channel: channel,
            auth_channel: auth_channel,
        }
    }
}

impl<T> Resource<T> {
    pub async fn create(&self, initial_data: String) -> SirixResult<SirixResponse<String>> {
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

    pub async fn exists(&self) -> SirixResult<SirixResponse<bool>> {
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
}

fn build_read_params(read_args: ReadArgs) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();
    match read_args.node_id {
        Some(node_id) => {
            params.push(("nodeId".to_owned(), node_id.to_string()));
        }
        None => (),
    };
    match read_args.max_level {
        Some(max_level) => {
            params.push(("maxLevel".to_owned(), max_level.to_string()));
        }
        None => (),
    };
    match read_args.top_level_limit {
        Some(top_level_limit) => {
            params.push(("nextTopLevelNodes".to_owned(), top_level_limit.to_string()));
        }
        None => (),
    };
    match read_args.top_level_skip_last_node {
        Some(top_level_skip_last_node) => {
            params.push((
                "lastTopLevelNodeKey".to_owned(),
                top_level_skip_last_node.to_string(),
            ));
        }
        None => (),
    };
    match read_args.revision {
        RevisionArg::SingleRevision(revision) => match revision {
            SingleRevision::Number(revision) => {
                params.push(("revision".to_owned(), revision.to_string()));
            }
            SingleRevision::Timestamp(revision) => {
                params.push(("revision-timestamp".to_owned(), revision.to_string()));
            }
        },
        RevisionArg::TwoRevisions(revisions) => {
            match revisions {
                TwoRevisions::Number(first_revision, second_revision) => {
                    params.push(("start-revision".to_owned(), first_revision.to_string()));
                    params.push(("end-revision".to_owned(), second_revision.to_string()));
                }
                TwoRevisions::Timestamp(first_revision, second_revision) => {
                    params.push(("start-revision-timestamp".to_owned(), first_revision));
                    params.push(("end-revision-timestamp".to_owned(), second_revision));
                }
            };
        }
    };
    return params;
}
