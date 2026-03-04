use serde::de::DeserializeOwned;

use super::super::info;
use super::super::types::{DbInfo, DbType, Json, Xml};
use super::client::SirixResponse;
use super::error::SirixResult;
use super::http::{create_database, delete_database, get_database_info, get_database_info_string};
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
        self.info_raw()
    }

    pub fn info_raw<U: DeserializeOwned>(&self) -> SirixResult<SirixResponse<U>> {
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

    pub fn info_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                get_database_info_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                )
            }
            None => {
                get_database_info_string(self.agent.clone(), None, &self.base_uri, &self.db_name)
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::test_mocks;
    use mockito::mock;
    use std::sync::{Arc, RwLock};

    fn base_url() -> String {
        mockito::server_url()
    }

    fn agent() -> ureq::Agent {
        ureq::agent()
    }

    fn make_auth_lock() -> Option<Arc<RwLock<Option<info::TokenData>>>> {
        Some(Arc::new(RwLock::new(Some(test_mocks::get_token_data()))))
    }

    // -- Database<Json> --

    #[test]
    fn json_database_new_sets_fields() {
        let db = Database::<Json>::new("mydb".to_string(), base_url(), agent(), None);
        assert_eq!(db.db_name, "mydb");
        assert!(matches!(db.db_type, DbType::Json(_)));
    }

    #[test]
    fn json_database_resource_returns_json_resource() {
        let db = Database::<Json>::new("mydb".to_string(), base_url(), agent(), None);
        let res = db.resource("myres".to_string());
        assert_eq!(res.db_name, "mydb");
        assert_eq!(res.resource_name, "myres");
    }

    #[test]
    fn json_database_create_without_auth() {
        let _m = mock("PUT", "/db_cr_json")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db =
            Database::<Json>::new("db_cr_json".to_string(), base_url(), agent(), None);
        let result = db.create();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_create_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("PUT", "/db_cr_json_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db = Database::<Json>::new(
            "db_cr_json_auth".to_string(),
            base_url(),
            agent(),
            make_auth_lock(),
        );
        let result = db.create();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_delete_without_auth() {
        let _m = mock("DELETE", "/db_del_json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db =
            Database::<Json>::new("db_del_json".to_string(), base_url(), agent(), None);
        let result = db.delete();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_delete_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("DELETE", "/db_del_json_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db = Database::<Json>::new(
            "db_del_json_auth".to_string(),
            base_url(),
            agent(),
            make_auth_lock(),
        );
        let result = db.delete();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_info_without_auth() {
        let _m = mock("GET", "/db_info_json")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"resources":["r1"]}"#)
            .create();

        let db = Database::<Json>::new(
            "db_info_json".to_string(),
            base_url(),
            agent(),
            None,
        );
        let result = db.info();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_info_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/db_info_json_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"resources":[]}"#)
            .create();

        let db = Database::<Json>::new(
            "db_info_json_auth".to_string(),
            base_url(),
            agent(),
            make_auth_lock(),
        );
        let result = db.info();
        assert!(result.is_ok());
    }

    #[test]
    fn json_database_info_string_without_auth() {
        let _m = mock("GET", "/db_info_str")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body(r#"{"resources":[]}"#)
            .create();

        let db =
            Database::<Json>::new("db_info_str".to_string(), base_url(), agent(), None);
        let result = db.info_string();
        let resp = result.unwrap();
        assert_eq!(resp.body, r#"{"resources":[]}"#);
    }

    #[test]
    fn json_database_info_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/db_info_str_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("info data")
            .create();

        let db = Database::<Json>::new(
            "db_info_str_auth".to_string(),
            base_url(),
            agent(),
            make_auth_lock(),
        );
        let result = db.info_string();
        assert!(result.is_ok());
    }

    // -- Database<Xml> --

    #[test]
    fn xml_database_new_sets_fields() {
        let db = Database::<Xml>::new("xmldb".to_string(), base_url(), agent(), None);
        assert_eq!(db.db_name, "xmldb");
        assert!(matches!(db.db_type, DbType::XML(_)));
    }

    #[test]
    fn xml_database_resource_returns_xml_resource() {
        let db = Database::<Xml>::new("xmldb".to_string(), base_url(), agent(), None);
        let res = db.resource("xmlres".to_string());
        assert_eq!(res.db_name, "xmldb");
        assert_eq!(res.resource_name, "xmlres");
    }

    #[test]
    fn xml_database_create() {
        let _m = mock("PUT", "/db_cr_xml")
            .match_header("content-type", "application/xml")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db =
            Database::<Xml>::new("db_cr_xml".to_string(), base_url(), agent(), None);
        let result = db.create();
        assert!(result.is_ok());
    }

    #[test]
    fn xml_database_delete() {
        let _m = mock("DELETE", "/db_del_xml")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let db =
            Database::<Xml>::new("db_del_xml".to_string(), base_url(), agent(), None);
        let result = db.delete();
        assert!(result.is_ok());
    }
}
