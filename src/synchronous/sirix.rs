use crate::types::{Json, Xml};

use super::super::info;
use super::super::types::{InfoResults, InfoResultsWithResourcesContainer};
use super::client::SirixResponse;
use super::database::Database;
use super::error::SirixResult;
use super::http::{
    delete_all, global_info, global_info_string, global_info_with_resources,
    global_info_with_resources_string,
};
use serde::de::DeserializeOwned;
use std::{sync::Arc, sync::RwLock};

#[derive(Debug, Clone)]
pub struct Sirix {
    /// the url for the SirixDB server
    base_uri: String,
    /// a ureq::Agent
    agent: ureq::Agent,
    /// a reference to authentication data
    auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
}

impl Sirix {
    pub fn new(
        base_uri: String,
        agent: ureq::Agent,
        auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
    ) -> Self {
        return Self {
            base_uri,
            agent,
            auth_lock,
        };
    }

    pub fn json_database(&self, db_name: String) -> Database<Json> {
        Database::<Json>::new(
            db_name,
            self.base_uri.clone(),
            self.agent.clone(),
            self.auth_lock.clone(),
        )
    }

    pub fn xml_database(&self, db_name: String) -> Database<Xml> {
        Database::<Xml>::new(
            db_name,
            self.base_uri.clone(),
            self.agent.clone(),
            self.auth_lock.clone(),
        )
    }

    pub fn info(&self) -> SirixResult<SirixResponse<InfoResults>> {
        self.info_raw()
    }

    pub fn info_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                )
            }
            None => global_info(self.agent.clone(), None, &self.base_uri),
        }
    }

    fn _info_string(&self, xml: bool) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    xml,
                )
            }
            None => global_info_string(self.agent.clone(), None, &self.base_uri, false),
        }
    }

    pub fn info_string_json(&self) -> SirixResult<SirixResponse<String>> {
        self._info_string(false)
    }

    pub fn info_string_xml(&self) -> SirixResult<SirixResponse<String>> {
        self._info_string(true)
    }

    pub fn info_with_resources(
        &self,
    ) -> SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> {
        self.info_with_resources_raw()
    }

    pub fn info_with_resources_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info_with_resources(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                )
            }
            None => global_info_with_resources(self.agent.clone(), None, &self.base_uri),
        }
    }

    fn _info_with_resources_string(&self, xml: bool) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info_with_resources_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    xml,
                )
            }
            None => {
                global_info_with_resources_string(self.agent.clone(), None, &self.base_uri, xml)
            }
        }
    }

    pub fn _info_with_resources_xml(&self) -> SirixResult<SirixResponse<String>> {
        self._info_with_resources_string(true)
    }

    pub fn _info_with_resources_json(&self) -> SirixResult<SirixResponse<String>> {
        self._info_with_resources_string(false)
    }

    pub fn delete_all(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                delete_all(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                )
            }
            None => delete_all(self.agent.clone(), None, &self.base_uri),
        }
    }
}
