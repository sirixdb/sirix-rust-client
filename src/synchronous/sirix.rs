use crate::types::{Json, Xml};

use super::super::info;
use super::super::types::{InfoResults, InfoResultsWithResourcesContainer};
use super::client::SirixResponse;
use super::database::Database;
use super::error::SirixResult;
use super::http::{delete_all, global_info, global_info_with_resources};
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

    pub fn info_with_resources(
        &self,
    ) -> SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> {
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