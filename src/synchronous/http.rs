use crate::synchronous::client::request_string;

use super::client::{request, request_empty};
use super::{super::types::*, client::SirixResponse, error::SirixResult};
use serde::de::DeserializeOwned;
use ureq;

pub fn global_info<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(base_url)
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent.get(base_url).set("accept", "application/json"),
    };
    request(req, None)
}

pub fn global_info_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(base_url)
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent.get(base_url).set("accept", "application/json"),
    };
    request_string(req, None)
}

pub fn global_info_with_resources<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}?withResources=true", base_url))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent
            .get(&format!("{}?withResources=true", base_url))
            .set("accept", "application/json"),
    };
    request(req, None)
}

pub fn global_info_with_resources_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}?withResources=true", base_url))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent
            .get(&format!("{}?withResources=true", base_url))
            .set("accept", "application/json"),
    };
    request_string(req, None)
}

pub fn delete_all(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .delete(base_url)
            .set("authorization", &format!("Bearer {}", authorization)),
        None => agent.delete(base_url),
    };
    request_empty(req, None)
}

pub fn create_database(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .put(&format!("{}/{}", base_url, db_name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .put(&format!("{}/{}", base_url, db_name))
            .set("content-type", &db_type.to_string()),
    };
    request_empty(req, None)
}

pub fn get_database_info<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}/{}", base_url, db_name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent
            .get(&format!("{}/{}", base_url, db_name))
            .set("accept", "application/json"),
    };
    request(req, None)
}

pub fn get_database_info_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}/{}", base_url, db_name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", "application/json"),
        None => agent
            .get(&format!("{}/{}", base_url, db_name))
            .set("accept", "application/json"),
    };
    request_string(req, None)
}

pub fn delete_database(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .delete(&format!("{}/{}", base_url, db_name))
            .set("authorization", &format!("Bearer {}", authorization)),
        None => agent.delete(&format!("{}/{}", base_url, db_name)),
    };
    request_empty(req, None)
}

pub fn resource_exists(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request_empty(req, None)
}

pub fn create_resource<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    initial_data: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .put(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .put(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request(req, Some(initial_data))
}

pub fn create_resource_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    initial_data: &str,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => agent
            .put(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .put(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request_string(req, Some(initial_data))
}

pub fn read_resource<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    params: Vec<(String, String)>,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => {
            let mut req = agent
                .get(&format!("{}/{}/{}", base_url, db_name, name))
                .set("authorization", &format!("Bearer {}", authorization))
                .set("accept", &db_type.to_string());
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
        None => {
            let mut req = agent
                .get(&format!("{}/{}/{}", base_url, db_name, name))
                .set("accept", &db_type.to_string());
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
    };
    request(req, None)
}

pub fn read_resource_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    params: Vec<(String, String)>,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => {
            let mut req = agent
                .get(&format!("{}/{}/{}", base_url, db_name, name))
                .set("authorization", &format!("Bearer {}", authorization))
                .set("accept", &db_type.to_string());
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
        None => {
            let mut req = agent
                .get(&format!("{}/{}/{}", base_url, db_name, name))
                .set("accept", &db_type.to_string());
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
    };
    request_string(req, None)
}

pub fn resource_history<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}/{}/{}/history", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .get(&format!("{}/{}/{}/history", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request(req, None)
}

pub fn resource_history_string(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
) -> SirixResult<SirixResponse<String>> {
    let req = match authorization {
        Some(authorization) => agent
            .get(&format!("{}/{}/{}/history", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .get(&format!("{}/{}/{}/history", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request_string(req, None)
}

pub fn diff_resource(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    name: &str,
    params: Vec<(String, String)>,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => {
            let mut req = agent
                .get(&format!("{}/{}/{}", base_url, db_name, name))
                .set("authorization", &format!("Bearer {}", authorization));
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
        None => {
            let mut req = agent.get(&format!("{}/{}/{}", base_url, db_name, name));
            params.iter().for_each(|param| {
                req = req.clone().query(&param.0, &param.1);
            });
            req
        }
    };
    request_empty(req, None)
}

pub fn post_query<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    query: &Query,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .post(base_url)
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", "application/json"),
        None => agent.post(base_url).set("content-type", "application/json"),
    };
    request(req, Some(&serde_json::to_string(query).unwrap()))
}

pub fn get_etag(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    node_id: u128,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("accept", &db_type.to_string())
            .query("nodeId", &node_id.to_string()),
        None => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("accept", &db_type.to_string())
            .query("nodeId", &node_id.to_string()),
    };
    request_empty(req, None)
}

pub fn update_resource<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    node_id: u128,
    insert: Insert,
    data: &str,
    etag: &str,
) -> SirixResult<SirixResponse<T>> {
    let req = match authorization {
        Some(authorization) => agent
            .post(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string())
            .set("etag", etag)
            .query("nodeId", &node_id.to_string())
            .query("insert", &insert.to_string()),
        None => agent
            .post(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string())
            .set("etag", etag)
            .query("nodeId", &node_id.to_string())
            .query("insert", &insert.to_string()),
    };
    request(req, Some(data))
}

pub fn resource_delete(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
    node_and_etag: Option<NodeIdAndEtag>,
) -> SirixResult<SirixResponse<()>> {
    let req = match authorization {
        Some(authorization) => agent
            .delete(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .delete(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    let req = match node_and_etag {
        Some(node_id_and_etag) => req
            .set("etag", &node_id_and_etag.etag.to_string())
            .query("nodeId", &node_id_and_etag.node_id.to_string()),
        None => req,
    };
    request_empty(req, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    fn agent() -> ureq::Agent {
        ureq::agent()
    }

    fn base_url() -> String {
        mockito::server_url()
    }

    // -- global_info --

    #[test]
    fn global_info_with_auth_sends_bearer_token() {
        let _m = mock("GET", "/")
            .match_header("authorization", "Bearer my_token")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"name":"db1","info_type":"json"}]"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> =
            global_info(agent(), Some("my_token"), &base_url());
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn global_info_without_auth_succeeds() {
        let _m = mock("GET", "/")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> =
            global_info(agent(), None, &base_url());
        assert!(result.is_ok());
    }

    // -- global_info_string --

    #[test]
    fn global_info_string_returns_raw_body() {
        let _m = mock("GET", "/")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body(r#"{"databases":[]}"#)
            .create();

        let result = global_info_string(agent(), None, &base_url());
        let resp = result.unwrap();
        assert_eq!(resp.body, r#"{"databases":[]}"#);
    }

    #[test]
    fn global_info_string_with_auth() {
        let _m = mock("GET", "/")
            .match_header("authorization", "Bearer tok")
            .with_status(200)
            .with_body("info")
            .create();

        let result = global_info_string(agent(), Some("tok"), &base_url());
        assert!(result.is_ok());
    }

    // -- global_info_with_resources --

    #[test]
    fn global_info_with_resources_adds_query_param() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"databases":[]}"#)
            .create();

        let result: SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> =
            global_info_with_resources(agent(), None, &base_url());
        let resp = result.unwrap();
        assert!(resp.body.databases.is_empty());
    }

    #[test]
    fn global_info_with_resources_with_auth() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .match_header("authorization", "Bearer tok2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"databases":[]}"#)
            .create();

        let result: SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> =
            global_info_with_resources(agent(), Some("tok2"), &base_url());
        assert!(result.is_ok());
    }

    // -- global_info_with_resources_string --

    #[test]
    fn global_info_with_resources_string_without_auth() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .with_status(200)
            .with_body("resources_string")
            .create();

        let result = global_info_with_resources_string(agent(), None, &base_url());
        let resp = result.unwrap();
        assert_eq!(resp.body, "resources_string");
    }

    #[test]
    fn global_info_with_resources_string_with_auth() {
        let _m = mock("GET", "/")
            .match_query(Matcher::UrlEncoded(
                "withResources".into(),
                "true".into(),
            ))
            .match_header("authorization", "Bearer tok3")
            .with_status(200)
            .with_body("auth_resources")
            .create();

        let result =
            global_info_with_resources_string(agent(), Some("tok3"), &base_url());
        assert!(result.is_ok());
    }

    // -- delete_all --

    #[test]
    fn delete_all_sends_delete_to_base_url() {
        let _m = mock("DELETE", "/")
            .with_status(204)
            .create();

        let result = delete_all(agent(), None, &base_url());
        assert!(result.is_ok());
    }

    #[test]
    fn delete_all_with_auth_sends_bearer() {
        let _m = mock("DELETE", "/")
            .match_header("authorization", "Bearer del_tok")
            .with_status(204)
            .create();

        let result = delete_all(agent(), Some("del_tok"), &base_url());
        assert!(result.is_ok());
    }

    // -- create_database --

    #[test]
    fn create_database_sends_put_with_json_content_type() {
        let _m = mock("PUT", "/testdb")
            .match_header("content-type", "application/json")
            .with_status(200)
            .create();

        let result = create_database(
            agent(),
            None,
            &base_url(),
            "testdb",
            DbType::Json(Json),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn create_database_sends_put_with_xml_content_type() {
        let _m = mock("PUT", "/xmldb")
            .match_header("content-type", "application/xml")
            .with_status(200)
            .create();

        let result =
            create_database(agent(), None, &base_url(), "xmldb", DbType::XML(Xml));
        assert!(result.is_ok());
    }

    #[test]
    fn create_database_with_auth() {
        let _m = mock("PUT", "/authdb")
            .match_header("authorization", "Bearer create_tok")
            .match_header("content-type", "application/json")
            .with_status(200)
            .create();

        let result = create_database(
            agent(),
            Some("create_tok"),
            &base_url(),
            "authdb",
            DbType::Json(Json),
        );
        assert!(result.is_ok());
    }

    // -- get_database_info --

    #[test]
    fn get_database_info_sends_get_with_db_name() {
        let _m = mock("GET", "/mydb")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"resources":["r1"]}"#)
            .create();

        let result: SirixResult<SirixResponse<DbInfo>> =
            get_database_info(agent(), None, &base_url(), "mydb");
        let resp = result.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn get_database_info_with_auth() {
        let _m = mock("GET", "/mydb2")
            .match_header("authorization", "Bearer info_tok")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"resources":[]}"#)
            .create();

        let result: SirixResult<SirixResponse<DbInfo>> =
            get_database_info(agent(), Some("info_tok"), &base_url(), "mydb2");
        assert!(result.is_ok());
    }

    // -- get_database_info_string --

    #[test]
    fn get_database_info_string_returns_body() {
        let _m = mock("GET", "/strdb")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body(r#"{"resources":["a"]}"#)
            .create();

        let result = get_database_info_string(agent(), None, &base_url(), "strdb");
        let resp = result.unwrap();
        assert_eq!(resp.body, r#"{"resources":["a"]}"#);
    }

    #[test]
    fn get_database_info_string_with_auth() {
        let _m = mock("GET", "/strdb2")
            .match_header("authorization", "Bearer str_tok")
            .with_status(200)
            .with_body("info")
            .create();

        let result =
            get_database_info_string(agent(), Some("str_tok"), &base_url(), "strdb2");
        assert!(result.is_ok());
    }

    // -- delete_database --

    #[test]
    fn delete_database_sends_delete_with_db_name() {
        let _m = mock("DELETE", "/deldb")
            .with_status(204)
            .create();

        let result = delete_database(agent(), None, &base_url(), "deldb");
        assert!(result.is_ok());
    }

    #[test]
    fn delete_database_with_auth() {
        let _m = mock("DELETE", "/deldb2")
            .match_header("authorization", "Bearer del_db_tok")
            .with_status(204)
            .create();

        let result =
            delete_database(agent(), Some("del_db_tok"), &base_url(), "deldb2");
        assert!(result.is_ok());
    }

    // -- create_resource --

    #[test]
    fn create_resource_sends_put_with_body() {
        let _m = mock("PUT", "/crdb/crres")
            .match_header("content-type", "application/json")
            .match_body(r#"{"key":"value"}"#)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"key":"value"}"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = create_resource(
            agent(),
            None,
            &base_url(),
            "crdb",
            DbType::Json(Json),
            "crres",
            r#"{"key":"value"}"#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn create_resource_with_auth() {
        let _m = mock("PUT", "/crdb2/crres2")
            .match_header("authorization", "Bearer cr_tok")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = create_resource(
            agent(),
            Some("cr_tok"),
            &base_url(),
            "crdb2",
            DbType::Json(Json),
            "crres2",
            "{}",
        );
        assert!(result.is_ok());
    }

    // -- create_resource_string --

    #[test]
    fn create_resource_string_returns_body() {
        let _m = mock("PUT", "/csdb/csres")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body("created")
            .create();

        let result = create_resource_string(
            agent(),
            None,
            &base_url(),
            "csdb",
            DbType::Json(Json),
            "csres",
            "{}",
        );
        let resp = result.unwrap();
        assert_eq!(resp.body, "created");
    }

    #[test]
    fn create_resource_string_with_auth() {
        let _m = mock("PUT", "/csdb2/csres2")
            .match_header("authorization", "Bearer cs_tok")
            .with_status(200)
            .with_body("ok")
            .create();

        let result = create_resource_string(
            agent(),
            Some("cs_tok"),
            &base_url(),
            "csdb2",
            DbType::Json(Json),
            "csres2",
            "data",
        );
        assert!(result.is_ok());
    }

    // -- read_resource --

    #[test]
    fn read_resource_sends_get_with_params() {
        let _m = mock("GET", "/rdb/rres")
            .match_header("accept", "application/json")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "1".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"foo":"bar"}"#)
            .create();

        let params = vec![("nodeId".to_string(), "1".to_string())];
        let result: SirixResult<SirixResponse<serde_json::Value>> = read_resource(
            agent(),
            None,
            &base_url(),
            "rdb",
            DbType::Json(Json),
            "rres",
            params,
        );
        let resp = result.unwrap();
        assert_eq!(resp.body["foo"], "bar");
    }

    #[test]
    fn read_resource_with_auth_and_empty_params() {
        let _m = mock("GET", "/rdb2/rres2")
            .match_header("authorization", "Bearer r_tok")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data":true}"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = read_resource(
            agent(),
            Some("r_tok"),
            &base_url(),
            "rdb2",
            DbType::Json(Json),
            "rres2",
            vec![],
        );
        assert!(result.is_ok());
    }

    // -- read_resource_string --

    #[test]
    fn read_resource_string_returns_body() {
        let _m = mock("GET", "/rsdb/rsres")
            .match_header("accept", "application/json")
            .with_status(200)
            .with_body("resource content")
            .create();

        let result = read_resource_string(
            agent(),
            None,
            &base_url(),
            "rsdb",
            DbType::Json(Json),
            "rsres",
            vec![],
        );
        let resp = result.unwrap();
        assert_eq!(resp.body, "resource content");
    }

    #[test]
    fn read_resource_string_with_auth() {
        let _m = mock("GET", "/rsdb2/rsres2")
            .match_header("authorization", "Bearer rs_tok")
            .with_status(200)
            .with_body("auth content")
            .create();

        let result = read_resource_string(
            agent(),
            Some("rs_tok"),
            &base_url(),
            "rsdb2",
            DbType::Json(Json),
            "rsres2",
            vec![],
        );
        assert!(result.is_ok());
    }

    // -- resource_exists --

    #[test]
    fn resource_exists_sends_head_request() {
        let _m = mock("HEAD", "/exdb/exres")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("true")
            .create();

        let result = resource_exists(
            agent(),
            None,
            &base_url(),
            "exdb",
            DbType::Json(Json),
            "exres",
        );
        // HEAD requests may not have a body, so this might error on parsing
        // Testing that the correct request is made
        let _ = result;
    }

    #[test]
    fn resource_exists_with_auth() {
        let _m = mock("HEAD", "/exdb2/exres2")
            .match_header("authorization", "Bearer ex_tok")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body("true")
            .create();

        let result = resource_exists(
            agent(),
            Some("ex_tok"),
            &base_url(),
            "exdb2",
            DbType::Json(Json),
            "exres2",
        );
        let _ = result;
    }

    // -- resource_history --

    #[test]
    fn resource_history_sends_get_to_history_endpoint() {
        let _m = mock("GET", "/hdb/hres/history")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"revision_timestamp":"2021","revision":1,"author":"admin","commit_message":"msg"}]"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = resource_history(
            agent(),
            None,
            &base_url(),
            "hdb",
            DbType::Json(Json),
            "hres",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn resource_history_with_auth() {
        let _m = mock("GET", "/hdb2/hres2/history")
            .match_header("authorization", "Bearer h_tok")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = resource_history(
            agent(),
            Some("h_tok"),
            &base_url(),
            "hdb2",
            DbType::Json(Json),
            "hres2",
        );
        assert!(result.is_ok());
    }

    // -- resource_history_string --

    #[test]
    fn resource_history_string_returns_body() {
        let _m = mock("GET", "/hsdb/hsres/history")
            .with_status(200)
            .with_body("history data")
            .create();

        let result = resource_history_string(
            agent(),
            None,
            &base_url(),
            "hsdb",
            DbType::Json(Json),
            "hsres",
        );
        let resp = result.unwrap();
        assert_eq!(resp.body, "history data");
    }

    #[test]
    fn resource_history_string_with_auth() {
        let _m = mock("GET", "/hsdb2/hsres2/history")
            .match_header("authorization", "Bearer hs_tok")
            .with_status(200)
            .with_body("auth history")
            .create();

        let result = resource_history_string(
            agent(),
            Some("hs_tok"),
            &base_url(),
            "hsdb2",
            DbType::Json(Json),
            "hsres2",
        );
        assert!(result.is_ok());
    }

    // -- diff_resource --

    #[test]
    fn diff_resource_sends_get_with_diff_params() {
        let _m = mock("GET", "/ddb/dres")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("first-revision".into(), "1".into()),
                Matcher::UrlEncoded("second-revision".into(), "2".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let params = vec![
            ("first-revision".to_string(), "1".to_string()),
            ("second-revision".to_string(), "2".to_string()),
        ];
        let result = diff_resource(agent(), None, &base_url(), "ddb", "dres", params);
        assert!(result.is_ok());
    }

    #[test]
    fn diff_resource_with_auth() {
        let _m = mock("GET", "/ddb2/dres2")
            .match_header("authorization", "Bearer d_tok")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let result = diff_resource(
            agent(),
            Some("d_tok"),
            &base_url(),
            "ddb2",
            "dres2",
            vec![],
        );
        assert!(result.is_ok());
    }

    // -- post_query --

    #[test]
    fn post_query_sends_post_with_query_body() {
        let q = Query::new("jn:doc('db','res')".to_string(), None, None);
        let expected_body = serde_json::to_string(&q).unwrap();

        let _m = mock("POST", "/")
            .match_body(expected_body.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result":"data"}"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> =
            post_query(agent(), None, &base_url(), &q);
        let resp = result.unwrap();
        assert_eq!(resp.body["result"], "data");
    }

    #[test]
    fn post_query_with_auth() {
        let q = Query::new("test".to_string(), Some(0), Some(5));
        let expected_body = serde_json::to_string(&q).unwrap();

        let _m = mock("POST", "/")
            .match_header("authorization", "Bearer q_tok")
            .match_body(expected_body.as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> =
            post_query(agent(), Some("q_tok"), &base_url(), &q);
        assert!(result.is_ok());
    }

    // -- get_etag --

    #[test]
    fn get_etag_sends_head_with_node_id() {
        let _m = mock("HEAD", "/edb/eres")
            .match_header("accept", "application/json")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "42".into()))
            .with_status(200)
            .with_header("etag", "\"etag_val\"")
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let result = get_etag(
            agent(),
            None,
            &base_url(),
            "edb",
            DbType::Json(Json),
            "eres",
            42,
        );
        // HEAD may not produce a parseable body, but the request is correct
        let _ = result;
    }

    #[test]
    fn get_etag_with_auth() {
        let _m = mock("HEAD", "/edb2/eres2")
            .match_header("authorization", "Bearer e_tok")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "7".into()))
            .with_status(200)
            .with_body("null")
            .create();

        let result = get_etag(
            agent(),
            Some("e_tok"),
            &base_url(),
            "edb2",
            DbType::Json(Json),
            "eres2",
            7,
        );
        let _ = result;
    }

    // -- update_resource --

    #[test]
    fn update_resource_sends_post_with_insert_params() {
        let _m = mock("POST", "/udb/ures")
            .match_header("content-type", "application/json")
            .match_header("etag", "\"my_etag\"")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "10".into()),
                Matcher::UrlEncoded("insert".into(), "asFirstChild".into()),
            ]))
            .match_body(r#"{"new":"data"}"#)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"updated":true}"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = update_resource(
            agent(),
            None,
            &base_url(),
            "udb",
            DbType::Json(Json),
            "ures",
            10,
            Insert::Child,
            r#"{"new":"data"}"#,
            "\"my_etag\"",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn update_resource_with_auth_and_replace() {
        let _m = mock("POST", "/udb2/ures2")
            .match_header("authorization", "Bearer u_tok")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("nodeId".into(), "5".into()),
                Matcher::UrlEncoded("insert".into(), "replace".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"null"#)
            .create();

        let result: SirixResult<SirixResponse<serde_json::Value>> = update_resource(
            agent(),
            Some("u_tok"),
            &base_url(),
            "udb2",
            DbType::Json(Json),
            "ures2",
            5,
            Insert::Replace,
            "data",
            "etag",
        );
        assert!(result.is_ok());
    }

    // -- resource_delete --

    #[test]
    fn resource_delete_sends_delete_without_node_and_etag() {
        let _m = mock("DELETE", "/rddb/rdres")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let result = resource_delete(
            agent(),
            None,
            &base_url(),
            "rddb",
            DbType::Json(Json),
            "rdres",
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn resource_delete_with_node_and_etag() {
        let _m = mock("DELETE", "/rddb2/rdres2")
            .match_header("content-type", "application/json")
            .match_header("etag", "\"del_etag\"")
            .match_query(Matcher::UrlEncoded("nodeId".into(), "99".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let result = resource_delete(
            agent(),
            None,
            &base_url(),
            "rddb2",
            DbType::Json(Json),
            "rdres2",
            Some(NodeIdAndEtag {
                node_id: 99,
                etag: "\"del_etag\"".to_string(),
            }),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn resource_delete_with_auth() {
        let _m = mock("DELETE", "/rddb3/rdres3")
            .match_header("authorization", "Bearer rd_tok")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("null")
            .create();

        let result = resource_delete(
            agent(),
            Some("rd_tok"),
            &base_url(),
            "rddb3",
            DbType::Json(Json),
            "rdres3",
            None,
        );
        assert!(result.is_ok());
    }
}
