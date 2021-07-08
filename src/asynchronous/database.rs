//! Working with a Sirix database.

use super::super::info::TokenData;
use super::super::types::{DbInfo, DbType, Json, Xml};
use super::client::{Message, SirixResponse};
use super::http::{create_database, delete_database, get_database_info};
use super::resource::Resource;
use super::SirixResult;
use hyper::http::uri::{Authority, Scheme};
use tokio::sync::mpsc::Sender;
use tokio::sync::watch::Receiver;

///  Struct for manipulating a resource
#[derive(Debug, Clone)]
pub struct Database<T> {
    _t: T,
    /// The name of the database this resource belongs to.
    pub db_name: String,
    /// The type of that database.
    pub db_type: DbType,
    /// the scheme with which to access the SirixDB server
    scheme: Scheme,
    /// the authority at which to access the SirixDB server
    authority: Authority,
    /// the message channel for sending HTTP requests
    channel: Sender<Message>,
    /// the channel containing authentication data
    auth_channel: Option<Receiver<Option<TokenData>>>,
}

impl<T> Database<T> {
    pub async fn info(&self) -> SirixResult<SirixResponse<DbInfo>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                get_database_info(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                get_database_info(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn delete(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                delete_database(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                delete_database(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }

    pub async fn create(&self) -> SirixResult<SirixResponse<()>> {
        match self.auth_channel.clone() {
            Some(watcher) => {
                let token_data = watcher.borrow().as_ref().unwrap().clone();
                let token = token_data.token_type + " " + &token_data.access_token;
                create_database(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    Some(&token),
                    self.channel.clone(),
                )
                .await
            }
            None => {
                create_database(
                    self.scheme.clone(),
                    self.authority.clone(),
                    &self.db_name,
                    self.db_type.clone(),
                    None,
                    self.channel.clone(),
                )
                .await
            }
        }
    }
}

impl Database<Json> {
    pub fn new(
        db_name: String,
        scheme: Scheme,
        authority: Authority,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<TokenData>>>,
    ) -> Self {
        Self {
            _t: Json,
            db_name,
            db_type: DbType::Json(Json),
            scheme,
            authority,
            channel,
            auth_channel,
        }
    }

    /// a helper function to create a Resource struct corresponding
    /// to a resource within the database this Database struct represents
    pub fn resource(&self, name: String) -> Resource<Json> {
        Resource::<Json>::new(
            self.db_name.clone(),
            name,
            self.scheme.clone(),
            self.authority.clone(),
            self.channel.clone(),
            self.auth_channel.clone(),
        )
    }
}

impl Database<Xml> {
    pub fn new(
        db_name: String,
        scheme: Scheme,
        authority: Authority,
        channel: Sender<Message>,
        auth_channel: Option<Receiver<Option<TokenData>>>,
    ) -> Self {
        Self {
            _t: Xml,
            db_name,
            db_type: DbType::XML(Xml),
            scheme,
            authority,
            channel,
            auth_channel,
        }
    }

    /// a helper function to create a Resource struct corresponding
    /// to a resource within the database this Database struct represents
    pub fn resource(&self, name: String) -> Resource<Xml> {
        Resource::<Xml>::new(
            self.db_name.clone(),
            name,
            self.scheme.clone(),
            self.authority.clone(),
            self.channel.clone(),
            self.auth_channel.clone(),
        )
    }
}
