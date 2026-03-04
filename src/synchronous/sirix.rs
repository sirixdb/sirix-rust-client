use crate::types::{Json, Xml};

use super::super::info;
use super::super::types::{InfoResults, InfoResultsWithResourcesContainer, Query};
use super::client::SirixResponse;
use super::database::Database;
use super::error::SirixResult;
use super::http::{
    delete_all, global_info, global_info_string, global_info_with_resources,
    global_info_with_resources_string, post_query, post_query_string,
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

    pub fn info_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                )
            }
            None => global_info_string(self.agent.clone(), None, &self.base_uri),
        }
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

    pub fn info_with_resources_string(&self) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                global_info_with_resources_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                )
            }
            None => global_info_with_resources_string(self.agent.clone(), None, &self.base_uri),
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

    pub fn query<U: DeserializeOwned>(&self, query: Query) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                post_query(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &query,
                )
            }
            None => post_query(self.agent.clone(), None, &self.base_uri, &query),
        }
    }

    pub fn query_string(&self, query: Query) -> SirixResult<SirixResponse<String>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                post_query_string(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &query,
                )
            }
            None => post_query_string(self.agent.clone(), None, &self.base_uri, &query),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::test_mocks;
    use mockito::{mock, Matcher};
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

    #[test]
    fn new_creates_sirix_instance() {
        let sirix = Sirix::new(base_url(), agent(), None);
        assert_eq!(sirix.base_uri, base_url());
        assert!(sirix.auth_lock.is_none());
    }

    #[test]
    fn json_database_returns_json_typed_database() {
        let sirix = Sirix::new(base_url(), agent(), None);
        let db = sirix.json_database("testdb".to_string());
        assert_eq!(db.db_name, "testdb");
    }

    #[test]
    fn xml_database_returns_xml_typed_database() {
        let sirix = Sirix::new(base_url(), agent(), None);
        let db = sirix.xml_database("xmldb".to_string());
        assert_eq!(db.db_name, "xmldb");
    }

    #[test]
    fn info_without_auth_makes_request() {
        let _m = mock("GET", "/")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result = sirix.info();
        assert!(result.is_ok());
    }

    #[test]
    fn info_with_auth_sends_bearer_token() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/")
            .match_header("authorization", expected_auth.as_str())
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result = sirix.info();
        assert!(result.is_ok());
    }

    #[test]
    fn info_string_without_auth() {
        let _m = mock("GET", "/")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body(r#"{"databases":[]}"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result = sirix.info_string();
        let resp = result.unwrap();
        assert_eq!(resp.body, r#"{"databases":[]}"#);
    }

    #[test]
    fn info_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("info_data")
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result = sirix.info_string();
        assert!(result.is_ok());
    }

    #[test]
    fn info_with_resources_without_auth() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"databases":[]}"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result = sirix.info_with_resources();
        let resp = result.unwrap();
        assert!(resp.body.databases.is_empty());
    }

    #[test]
    fn info_with_resources_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"databases":[]}"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result = sirix.info_with_resources();
        assert!(result.is_ok());
    }

    #[test]
    fn info_with_resources_string_without_auth() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_body("resources_str")
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result = sirix.info_with_resources_string();
        let resp = result.unwrap();
        assert_eq!(resp.body, "resources_str");
    }

    #[test]
    fn info_with_resources_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_body("auth_res_str")
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result = sirix.info_with_resources_string();
        assert!(result.is_ok());
    }

    #[test]
    fn delete_all_without_auth() {
        let _m = mock("DELETE", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result = sirix.delete_all();
        assert!(result.is_ok());
    }

    #[test]
    fn delete_all_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("DELETE", "/")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result = sirix.delete_all();
        assert!(result.is_ok());
    }

    #[test]
    fn query_without_auth() {
        let q = Query::new("jn:doc('db','res')".to_string(), None, None);
        let expected_body = serde_json::to_string(&q).unwrap();

        let _m = mock("POST", "/")
            .match_body(expected_body.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result": 42}"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), None);
        let result: SirixResult<SirixResponse<serde_json::Value>> = sirix.query(q);
        let resp = result.unwrap();
        assert_eq!(resp.body["result"], 42);
    }

    #[test]
    fn query_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);
        let q = Query::new("test".to_string(), None, None);
        let expected_body = serde_json::to_string(&q).unwrap();

        let _m = mock("POST", "/")
            .match_header("authorization", expected_auth.as_str())
            .match_body(expected_body.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let sirix = Sirix::new(base_url(), agent(), make_auth_lock());
        let result: SirixResult<SirixResponse<serde_json::Value>> = sirix.query(q);
        assert!(result.is_ok());
    }
}
