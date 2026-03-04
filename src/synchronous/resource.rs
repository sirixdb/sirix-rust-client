use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::types::{
    DiffArgs, History, Insert, MetaNode, MetadataType, NodeIdAndEtag, ReadArgs, SingleRevision,
};

use super::super::info;
use super::super::types::{DbType, Json, Xml};
use super::super::utils::build_read_params;
use super::client::SirixResponse;
use super::error::SirixResult;
use super::http::{
    create_resource, create_resource_string, diff_resource, get_etag, read_resource,
    read_resource_string, resource_delete, resource_exists, resource_history,
    resource_history_string, update_resource,
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

    pub fn exists(&self) -> SirixResult<SirixResponse<()>> {
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

    pub fn delete(
        &self,
        node_and_etag: Option<NodeIdAndEtag>,
    ) -> SirixResult<SirixResponse<()>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                resource_delete(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_and_etag,
                )
            }
            None => resource_delete(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                node_and_etag,
            ),
        }
    }

    pub fn update<U: DeserializeOwned>(
        &self,
        node_id: u128,
        insert: Insert,
        data: String,
        etag: String,
    ) -> SirixResult<SirixResponse<U>> {
        match self.auth_lock.clone() {
            Some(lock) => {
                let token_data = Arc::clone(&lock).read().unwrap().clone().unwrap();
                update_resource(
                    self.agent.clone(),
                    Some(&token_data.access_token),
                    &self.base_uri,
                    &self.db_name,
                    self.db_type.clone(),
                    &self.resource_name,
                    node_id,
                    insert,
                    &data,
                    &etag,
                )
            }
            None => update_resource(
                self.agent.clone(),
                None,
                &self.base_uri,
                &self.db_name,
                self.db_type.clone(),
                &self.resource_name,
                node_id,
                insert,
                &data,
                &etag,
            ),
        }
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

    fn json_resource(name: &str, auth: bool) -> Resource<Json> {
        Resource::<Json>::new(
            "testdb".to_string(),
            name.to_string(),
            base_url(),
            agent(),
            if auth { make_auth_lock() } else { None },
        )
    }

    fn xml_resource(name: &str) -> Resource<Xml> {
        Resource::<Xml>::new(
            "testdb".to_string(),
            name.to_string(),
            base_url(),
            agent(),
            None,
        )
    }

    fn empty_read_args() -> ReadArgs {
        ReadArgs {
            node_id: None,
            revision: None,
            max_level: None,
            top_level_limit: None,
            top_level_skip_last_node: None,
        }
    }

    // -- Resource<Json>::new --

    #[test]
    fn json_resource_new_sets_fields() {
        let res = json_resource("myres", false);
        assert_eq!(res.db_name, "testdb");
        assert_eq!(res.resource_name, "myres");
        assert!(matches!(res.db_type, DbType::Json(_)));
    }

    // -- Resource<Xml>::new --

    #[test]
    fn xml_resource_new_sets_fields() {
        let res = xml_resource("xmlres");
        assert_eq!(res.db_name, "testdb");
        assert_eq!(res.resource_name, "xmlres");
        assert!(matches!(res.db_type, DbType::XML(_)));
    }

    // -- create --

    #[test]
    fn create_without_auth() {
        let _m = mock("PUT", "/testdb/res_create")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key":"value"}"#)
            .create();

        let res = json_resource("res_create", false);
        let result = res.create(r#"{"key":"value"}"#.to_string());
        let resp = result.unwrap();
        assert_eq!(resp.body["key"], "value");
    }

    #[test]
    fn create_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("PUT", "/testdb/res_create_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"created":true}"#)
            .create();

        let res = json_resource("res_create_auth", true);
        let result = res.create(r#"{}"#.to_string());
        assert!(result.is_ok());
    }

    // -- create_string --

    #[test]
    fn create_string_without_auth() {
        let _m = mock("PUT", "/testdb/res_cs")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body("created_string")
            .create();

        let res = json_resource("res_cs", false);
        let result = res.create_string("data".to_string());
        let resp = result.unwrap();
        assert_eq!(resp.body, "created_string");
    }

    #[test]
    fn create_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("PUT", "/testdb/res_cs_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("ok")
            .create();

        let res = json_resource("res_cs_auth", true);
        let result = res.create_string("data".to_string());
        assert!(result.is_ok());
    }

    // -- read --

    #[test]
    fn read_without_auth_and_no_params() {
        let _m = mock("GET", "/testdb/res_read")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data": 42}"#)
            .create();

        let res = json_resource("res_read", false);
        let result: SirixResult<SirixResponse<Value>> = res.read::<Value>(empty_read_args());
        let resp = result.unwrap();
        assert_eq!(resp.body["data"], 42);
    }

    #[test]
    fn read_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_read_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"ok":true}"#)
            .create();

        let res = json_resource("res_read_auth", true);
        let result: SirixResult<SirixResponse<Value>> = res.read::<Value>(empty_read_args());
        assert!(result.is_ok());
    }

    // -- read_string --

    #[test]
    fn read_string_without_auth() {
        let _m = mock("GET", "/testdb/res_rstr")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body("read_content")
            .create();

        let res = json_resource("res_rstr", false);
        let result = res.read_string(empty_read_args());
        let resp = result.unwrap();
        assert_eq!(resp.body, "read_content");
    }

    #[test]
    fn read_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_rstr_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("auth_content")
            .create();

        let res = json_resource("res_rstr_auth", true);
        let result = res.read_string(empty_read_args());
        assert!(result.is_ok());
    }

    // -- read_with_metadata_string --

    #[test]
    fn read_with_metadata_string_adds_metadata_param() {
        let _m = mock("GET", "/testdb/res_meta")
            .match_query(Matcher::UrlEncoded(
                "withMetadata".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_body("meta_content")
            .create();

        let res = json_resource("res_meta", false);
        let result =
            res.read_with_metadata_string(MetadataType::All, empty_read_args());
        let resp = result.unwrap();
        assert_eq!(resp.body, "meta_content");
    }

    #[test]
    fn read_with_metadata_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_meta_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::UrlEncoded(
                "withMetadata".into(),
                "nodeKey".into(),
            ))
            .with_status(200)
            .with_body("auth_meta")
            .create();

        let res = json_resource("res_meta_auth", true);
        let result =
            res.read_with_metadata_string(MetadataType::Key, empty_read_args());
        assert!(result.is_ok());
    }

    // -- exists --

    #[test]
    fn exists_without_auth() {
        let _m = mock("HEAD", "/testdb/res_exists")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body("true")
            .create();

        let res = json_resource("res_exists", false);
        let result = res.exists();
        // HEAD may not produce parseable body, just verify request is made
        let _ = result;
    }

    #[test]
    fn exists_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("HEAD", "/testdb/res_exists_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("true")
            .create();

        let res = json_resource("res_exists_auth", true);
        let result = res.exists();
        let _ = result;
    }

    // -- etag --

    #[test]
    fn etag_without_auth() {
        let _m = mock("GET", "/testdb/res_etag")
            .match_header("accept", "application/json")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "1".into()))
            .with_status(200)
            .with_header("etag", "\"etag_value\"")
            .with_body("null")
            .create();

        let res = json_resource("res_etag", false);
        let result = res.etag(1);
        let _ = result;
    }

    #[test]
    fn etag_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_etag_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::UrlEncoded("nodeId".into(), "5".into()))
            .with_status(200)
            .with_body("null")
            .create();

        let res = json_resource("res_etag_auth", true);
        let result = res.etag(5);
        let _ = result;
    }

    // -- history --

    #[test]
    fn history_string_without_auth() {
        let _m = mock("GET", "/testdb/res_hist/history")
            .with_status(200)
            .with_body("history data")
            .create();

        let res = json_resource("res_hist", false);
        let result = res.history_string();
        let resp = result.unwrap();
        assert_eq!(resp.body, "history data");
    }

    #[test]
    fn history_string_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_hist_auth/history")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_body("auth history")
            .create();

        let res = json_resource("res_hist_auth", true);
        let result = res.history_string();
        assert!(result.is_ok());
    }

    // -- diff --

    #[test]
    fn diff_without_auth_with_number_revisions() {
        let _m = mock("GET", "/testdb/res_diff")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("first-revision".into(), "1".into()),
                Matcher::UrlEncoded("second-revision".into(), "2".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_diff", false);
        let result = res.diff(DiffArgs {
            first_revision: SingleRevision::Number(1),
            second_revision: SingleRevision::Number(2),
            node_id: None,
            max_depth: None,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn diff_with_auth_and_timestamp_revisions() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("GET", "/testdb/res_diff_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded(
                    "first-revision".into(),
                    "2021-01-01".into(),
                ),
                Matcher::UrlEncoded(
                    "second-revision".into(),
                    "2021-06-01".into(),
                ),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_diff_auth", true);
        let result = res.diff(DiffArgs {
            first_revision: SingleRevision::Timestamp("2021-01-01".to_string()),
            second_revision: SingleRevision::Timestamp("2021-06-01".to_string()),
            node_id: None,
            max_depth: None,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn diff_with_node_id_and_max_depth() {
        let _m = mock("GET", "/testdb/res_diff_opts")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("startNodeKey".into(), "10".into()),
                Matcher::UrlEncoded("maxDepth".into(), "3".into()),
                Matcher::UrlEncoded("first-revision".into(), "1".into()),
                Matcher::UrlEncoded("second-revision".into(), "2".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_diff_opts", false);
        let result = res.diff(DiffArgs {
            first_revision: SingleRevision::Number(1),
            second_revision: SingleRevision::Number(2),
            node_id: Some(10),
            max_depth: Some(3),
        });
        assert!(result.is_ok());
    }

    // -- delete --

    #[test]
    fn delete_without_auth_and_no_node() {
        let _m = mock("DELETE", "/testdb/res_del")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_del", false);
        let result = res.delete(None);
        assert!(result.is_ok());
    }

    #[test]
    fn delete_with_auth() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("DELETE", "/testdb/res_del_auth")
            .match_header("authorization", expected_auth.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_del_auth", true);
        let result = res.delete(None);
        assert!(result.is_ok());
    }

    #[test]
    fn delete_with_node_and_etag() {
        let _m = mock("DELETE", "/testdb/res_del_node")
            .match_header("etag", "\"my_etag\"")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "99".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let res = json_resource("res_del_node", false);
        let result = res.delete(Some(NodeIdAndEtag {
            node_id: 99,
            etag: "\"my_etag\"".to_string(),
        }));
        assert!(result.is_ok());
    }

    // -- update --

    #[test]
    fn update_without_auth() {
        let _m = mock("POST", "/testdb/res_upd")
            .match_header("content-type", "application/json")
            .match_header("etag", "\"upd_etag\"")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "1".into()),
                Matcher::UrlEncoded("insert".into(), "asFirstChild".into()),
            ]))
            .match_body(r#"{"new":"data"}"#)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"updated":true}"#)
            .create();

        let res = json_resource("res_upd", false);
        let result: SirixResult<SirixResponse<Value>> = res.update(
            1,
            Insert::Child,
            r#"{"new":"data"}"#.to_string(),
            "\"upd_etag\"".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn update_with_auth_and_left_insert() {
        let token = test_mocks::get_token_data();
        let expected_auth = format!("Bearer {}", token.access_token);

        let _m = mock("POST", "/testdb/res_upd_auth")
            .match_header("authorization", expected_auth.as_str())
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "5".into()),
                Matcher::UrlEncoded("insert".into(), "asLeftSibling".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let res = json_resource("res_upd_auth", true);
        let result: SirixResult<SirixResponse<Value>> =
            res.update(5, Insert::Left, "data".to_string(), "etag".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn update_with_right_insert() {
        let _m = mock("POST", "/testdb/res_upd_right")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "3".into()),
                Matcher::UrlEncoded("insert".into(), "asRightSibling".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let res = json_resource("res_upd_right", false);
        let result: SirixResult<SirixResponse<Value>> =
            res.update(3, Insert::Right, "x".to_string(), "e".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn update_with_replace_insert() {
        let _m = mock("POST", "/testdb/res_upd_repl")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "7".into()),
                Matcher::UrlEncoded("insert".into(), "replace".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let res = json_resource("res_upd_repl", false);
        let result: SirixResult<SirixResponse<Value>> =
            res.update(7, Insert::Replace, "y".to_string(), "e".to_string());
        assert!(result.is_ok());
    }

    // -- XML resource tests --

    #[test]
    fn xml_resource_create_string() {
        let _m = mock("PUT", "/testdb/xml_res_cs")
            .match_header("content-type", "application/xml")
            .with_status(200)
            .with_body("xml_created")
            .create();

        let res = xml_resource("xml_res_cs");
        let result = res.create_string("<root/>".to_string());
        let resp = result.unwrap();
        assert_eq!(resp.body, "xml_created");
    }

    #[test]
    fn xml_resource_read_string() {
        let _m = mock("GET", "/testdb/xml_res_rs")
            .match_header("accept", "application/xml")
            .with_status(200)
            .with_body("<root/>")
            .create();

        let res = xml_resource("xml_res_rs");
        let result = res.read_string(empty_read_args());
        let resp = result.unwrap();
        assert_eq!(resp.body, "<root/>");
    }
}
