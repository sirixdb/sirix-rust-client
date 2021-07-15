use crate::synchronous::client::request_string;
use crate::types::InfoResultsWithResourcesContainer;

use super::client::request;
use super::{super::types::*, client::SirixResponse, error::SirixResult};
use serde::de::DeserializeOwned;
use ureq;

pub fn global_info(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<InfoResults>> {
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

pub fn global_info_with_resources(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
) -> SirixResult<SirixResponse<InfoResultsWithResourcesContainer>> {
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
    request(req, None)
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
    request(req, None)
}

pub fn get_database_info(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
) -> SirixResult<SirixResponse<DbInfo>> {
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
    request(req, None)
}

pub fn resource_exists(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    db_type: DbType,
    name: &str,
) -> SirixResult<SirixResponse<bool>> {
    let req = match authorization {
        Some(authorization) => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("authorization", &format!("Bearer {}", authorization))
            .set("content-type", &db_type.to_string()),
        None => agent
            .head(&format!("{}/{}/{}", base_url, db_name, name))
            .set("content-type", &db_type.to_string()),
    };
    request(req, None)
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

pub fn diff_resource<T: DeserializeOwned>(
    agent: ureq::Agent,
    authorization: Option<&str>,
    base_url: &str,
    db_name: &str,
    name: &str,
    params: Vec<(String, String)>,
) -> SirixResult<SirixResponse<T>> {
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
    request(req, None)
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
            .set("authorization", &format!("Bearer {}", authorization)),
        None => agent.post(base_url),
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
    request(req, None)
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
    request(req, None)
}
