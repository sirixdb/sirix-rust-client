use super::super::info;
use super::super::types::{DbInfo, DbType, Json, Xml};
use super::client::SirixResponse;
use super::error::SirixResult;
use super::http::{create_database, delete_database, get_database_info};
use super::resource::Resource;
use std::{sync::Arc, sync::RwLock};

#[derive(Debug, Clone)]
pub struct Database<T> {
    _t: T,
    /// The name of the database this resource belongs to.
    pub db_name: String,
    /// The type of that database.
    pub db_type: DbType,
    /// the url for the SirixDB server
    base_uri: String,
    /// a ureq::Agent
    agent: ureq::Agent,
    /// a reference to authentication data
    auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
}

impl<T> Database<T> {
    pub fn info(&self) -> SirixResult<SirixResponse<DbInfo>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                get_database_info(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                )
            }
            None => get_database_info(self.agent.clone(), None, &self.base_uri, &self.db_name),
        }
    }

    pub fn delete(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                delete_database(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                )
            }
            None => delete_database(self.agent.clone(), None, &self.base_uri, &self.db_name),
        }
    }

    pub fn create(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                create_database(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                )
            }
            None => create_database(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
            ),
        }
    }
}

impl Database<Json> {
    pub fn new(
        db_name: String,
        base_uri: String,
        agent: ureq::Agent,
        auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
    ) -> Self {
        Self {
            _t: Json,
            db_name: db_name,
            db_type: DbType::Json(Json),
            base_uri: base_uri,
            agent: agent,
            auth_lock: auth_lock,
        }
    }

    pub fn resource(&self, name: String) -> Resource<Json> {
        Resource::<Json>::new(
            self.db_name.clone(),
            name,
            self.base_uri.clone(),
            self.agent.clone(),
            self.auth_lock.clone(),
        )
    }
}

impl Database<Xml> {
    pub fn new(
        db_name: String,
        base_uri: String,
        agent: ureq::Agent,
        auth_lock: Option<Arc<RwLock<Option<info::TokenData>>>>,
    ) -> Self {
        Self {
            _t: Xml,
            db_name,
            db_type: DbType::XML(Xml),
            base_uri,
            agent,
            auth_lock,
        }
    }

    pub fn resource(&self, name: String) -> Resource<Xml> {
        Resource::<Xml>::new(
            self.db_name.clone(),
            name,
            self.base_uri.clone(),
            self.agent.clone(),
            self.auth_lock.clone(),
        )
    }
}
