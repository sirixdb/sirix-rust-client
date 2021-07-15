use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::types::{DiffArgs, History, MetaNode, MetadataType, ReadArgs, SingleRevision};

use super::super::info;
use super::super::types::{DbType, Json, Xml};
use super::super::utils::build_read_params;
use super::client::SirixResponse;
use super::error::SirixResult;
use super::http::{
    create_resource, create_resource_string, diff_resource, get_etag, read_resource,
    read_resource_string, resource_exists, resource_history, resource_history_string,
};
use std::{sync::Arc, sync::RwLock};

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
    /// the url for the SirixDB server
    base_uri: String,
    /// a ureq::Agent
    agent: ureq::Agent,
    /// a reference to authentication data
    auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
}

impl<T> Resource<T> {
    pub fn create_string(&self, initial_data: String) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                create_resource_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    &initial_data,
                )
            }
            None => create_resource_string(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                &initial_data,
            ),
        }
    }

    pub fn create_raw<U: DeserializeOwned>(
        &self,
        initial_data: String,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                create_resource(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    &initial_data,
                )
            }
            None => create_resource(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                &initial_data,
            ),
        }
    }

    pub fn create(&self, initial_data: String) -> SirixResult<SirixResponse<Value>> {
        self.create_raw(initial_data)
    }

    pub fn exists(&self) -> SirixResult<SirixResponse<bool>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                resource_exists(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                )
            }
            None => resource_exists(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
            ),
        }
    }

    pub fn etag(&self, node_id: u128) -> SirixResult<SirixResponse<()>> {
        let response = match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                get_etag(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                )
            }
            None => get_etag(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                node_id,
            ),
        };
        return response;
    }

    pub fn read<U: DeserializeOwned>(
        &self,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<Value>> {
        self.read_raw(read_args)
    }

    pub fn read_raw<U: DeserializeOwned>(
        &self,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<U>> {
        let params = build_read_params(read_args);
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                read_resource(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                )
            }
            None => read_resource(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                params,
            ),
        }
    }

    pub fn read_string(&self, read_args: ReadArgs) -> SirixResult<SirixResponse<String>> {
        let params = build_read_params(read_args);
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                read_resource_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                )
            }
            None => read_resource_string(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                params,
            ),
        }
    }

    pub fn read_with_metadata_string(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<String>> {
        let mut params = build_read_params(read_args);
        params.push(("withMetadata".to_owned(), meta_type.to_string()));
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                read_resource_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                )
            }
            None => read_resource_string(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                params,
            ),
        }
    }

    pub fn read_with_metadata_raw<U: DeserializeOwned>(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<U>> {
        let mut params = build_read_params(read_args);
        params.push(("withMetadata".to_owned(), meta_type.to_string()));
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                read_resource(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    params,
                )
            }
            None => read_resource(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                params,
            ),
        }
    }

    pub fn read_with_metadata(
        &self,
        meta_type: MetadataType,
        read_args: ReadArgs,
    ) -> SirixResult<SirixResponse<MetaNode>> {
        self.read_with_metadata_raw(meta_type, read_args)
    }
}

impl Resource<Json> {
    pub fn new(
        db_name: String,
        resource_name: String,
        base_uri: String,
        agent: ureq::Agent,
        auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
    ) -> Self {
        Self {
            _t: Json,
            db_name,
            db_type: DbType::Json(Json),
            resource_name,
            base_uri,
            agent,
            auth_lock,
        }
    }

    pub fn history_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                resource_history_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                )
            }
            None => resource_history_string(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
            ),
        }
    }

    pub fn history_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                resource_history(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                )
            }
            None => resource_history(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
            ),
        }
    }

    pub fn history(&self) -> SirixResult<SirixResponse<History>> {
        self.history_raw()
    }

    // TODO fix return type
    pub fn diff(&self, args: DiffArgs) -> SirixResult<SirixResponse<()>> {
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
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                diff_resource(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    &self.resource_name,
                    params,
                )
            }
            None => diff_resource(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                &self.resource_name,
                params,
            ),
        }
    }
}

impl Resource<Xml> {
    pub fn new(
        db_name: String,
        resource_name: String,
        base_uri: String,
        agent: ureq::Agent,
        auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
    ) -> Self {
        Self {
            _t: Xml,
            db_name,
            db_type: DbType::XML(Xml),
            resource_name,
            base_uri,
            agent,
            auth_lock,
        }
    }
}
