//! The various types used in SirixDB transactions

use super::info::{NodeTypeContainer, NodeTypePrimitive};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// A single commit
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub revision_timestamp: String,
    pub revision: usize,
    pub author: String,
    pub commit_message: String,
}

#[derive(Debug, Deserialize)]
pub struct History(pub Vec<Commit>);

type Resources = Vec<String>;

/// database info struct
#[derive(Debug, Deserialize)]
pub struct DbInfo {
    pub resources: Resources,
}

/// A diff from a delete operation
#[derive(Debug)]
pub struct DeleteDiff {
    pub node_key: usize,
    pub dewey_id: String,
    pub depth: u64,
}

/// A result from the global info request
#[derive(Debug, Deserialize)]
pub struct InfoResult {
    pub name: String,
    #[serde(rename = "type")]
    pub info_type: String,
}

/// A result from the global info request, resources included
#[derive(Debug, Deserialize)]
pub struct InfoResultWithResources {
    pub name: String,
    #[serde(rename = "type")]
    pub info_type: String,
    pub resources: Resources,
}

/// A full list for the global info request, without resources
#[derive(Debug, Default, Deserialize)]
pub struct InfoResults(pub Vec<InfoResult>);

/// A full list for the global info request, with resources
//#[derive(Debug, Default, Deserialize)]
pub type InfoResultsWithResources = Vec<InfoResultWithResources>;

/// A full list for the global info request, with resources
#[derive(Debug, Default, Deserialize)]
pub struct InfoResultsWithResourcesContainer {
    pub databases: InfoResultsWithResources,
}

#[derive(Debug, Default, Serialize)]
pub struct Query {
    #[serde(rename = "startResultSeqIndex", skip_serializing_if = "Option::is_none")]
    start_result_seq_index: Option<u128>,
    #[serde(rename = "endResultSeqIndex", skip_serializing_if = "Option::is_none")]
    end_result_seq_index: Option<u128>,
    query: String,
}

impl Query {
    pub fn new(
        query: String,
        start_result_seq_index: Option<u128>,
        end_result_seq_index: Option<u128>,
    ) -> Self {
        Self {
            query,
            start_result_seq_index,
            end_result_seq_index,
        }
    }
}

pub struct NodeIdAndEtag {
    pub node_id: u128,
    pub etag: String,
}

pub enum RevisionArg {
    SingleRevision(SingleRevision),
    TwoRevisions(TwoRevisions),
}

pub enum SingleRevision {
    Timestamp(String),
    Number(u64),
}

pub enum TwoRevisions {
    Timestamp(String, String),
    Number(u64, u64),
}

pub struct ReadArgs {
    pub node_id: Option<u128>,
    pub revision: Option<RevisionArg>,
    pub max_level: Option<u64>,
    pub top_level_limit: Option<u64>,
    pub top_level_skip_last_node: Option<u64>,
}

pub struct DiffArgs {
    pub first_revision: SingleRevision,
    pub second_revision: SingleRevision,
    pub node_id: Option<u128>,
    pub max_depth: Option<u64>,
}

/// A diff from an insert operation
#[derive(Debug)]
pub struct InsertDiff {
    pub node_key: u128,
    pub insert_position_node_key: usize,
    pub insert_position: String,
    pub dewey_id: String,
    pub depth: u64,
    pub insert_type: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct MetadataPrimitive {
    pub node_key: usize,
    pub hash: isize,
    pub node_type: NodeTypePrimitive,
}

#[derive(Debug, Deserialize)]
pub struct MetadataContainer {
    pub node_key: usize,
    pub hash: isize,
    pub node_type: NodeTypeContainer,
    pub descendant_count: usize,
    pub child_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MetaNode {
    OjbectKey(MetaNodeObjectKey),
    Array(MetaNodeArray),
    Object(MetaNodeObject),
    String(MetaNodeString),
    Number(MetaNodeNumber),
    Bool(MetaNodeBool),
    Null(MetaNodeNull),
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeObjectKey {
    pub metadata: MetadataPrimitive,
    pub key: String,
    pub value: Box<MetaNode>,
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeObject {
    pub metadata: MetadataContainer,
    #[serde(deserialize_with = "from_list_or_empty_object")]
    pub value: Vec<MetaNode>,
}

fn from_list_or_empty_object<'de, D>(deserializer: D) -> Result<Vec<MetaNode>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.chars().nth(0).unwrap().eq(&'{') {
        Ok(Vec::new())
    } else {
        let vec: Result<Vec<MetaNode>, serde_json::Error> = serde_json::from_str(s);
        match vec {
            Ok(val) => Ok(val),
            Err(err) => Err(serde::de::Error::custom(err)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeArray {
    pub metadata: MetadataContainer,
    pub value: Vec<MetaNode>,
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeString {
    pub metadata: MetadataPrimitive,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeNumber {
    pub metadata: MetadataPrimitive,
    pub value: serde_json::Number,
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeBool {
    pub metadata: MetadataPrimitive,
    pub value: bool,
}

#[derive(Debug, Deserialize)]
pub struct MetaNodeNull {
    pub metadata: MetadataPrimitive,
}

/// The result returned from a query
#[derive(Debug)]
pub struct QueryResult {
    pub revision_number: usize,
    pub revision_timestamp: String,
    pub revision: Revision,
}

/// A diff from a replace operation
#[derive(Debug)]
pub struct ReplaceDiff {
    pub node_key: usize,
    pub replace_type: String,
    pub data: String,
}

/// A timestamped revision ID
#[derive(Debug)]
pub struct Revision {
    pub timestamp: String,
    pub revision: usize,
}

/// All possible options for a resource update
#[derive(Debug)]
pub enum Insert {
    Child,
    Left,
    Right,
    Replace,
}

impl fmt::Display for Insert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Insert::*;
        let sirix_op = match self {
            Child => "asFirstChild",
            Left => "asLeftSibling",
            Right => "asRightSibling",
            Replace => "replace",
        };
        write!(f, "{}", sirix_op)
    }
}

#[derive(Debug, Clone)]
pub struct Xml;
#[derive(Debug, Clone)]
pub struct Json;

/// All possible options database (and resource) types
#[derive(Debug, Clone)]
pub enum DbType {
    Json(Json),
    XML(Xml),
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DbType::*;
        let db_type = match self {
            Json(_) => "application/json",
            XML(_) => "application/xml",
        };
        write!(f, "{}", db_type)
    }
}

/// The scope of the metadata to return using `readWithMetadata`
// TODO doc link to actual method
#[derive(Debug)]
pub enum MetadataType {
    All,
    Key,
    KeyAndChild,
}

impl fmt::Display for MetadataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MetadataType::*;
        let db_type = match self {
            All => "true", // bool??
            Key => "nodeKey",
            KeyAndChild => "nodeKeyAndChildCount",
        };
        write!(f, "{}", db_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_child_displays_as_first_child() {
        assert_eq!(Insert::Child.to_string(), "asFirstChild");
    }

    #[test]
    fn insert_left_displays_as_left_sibling() {
        assert_eq!(Insert::Left.to_string(), "asLeftSibling");
    }

    #[test]
    fn insert_right_displays_as_right_sibling() {
        assert_eq!(Insert::Right.to_string(), "asRightSibling");
    }

    #[test]
    fn insert_replace_displays_as_replace() {
        assert_eq!(Insert::Replace.to_string(), "replace");
    }

    #[test]
    fn db_type_json_displays_as_json_content_type() {
        assert_eq!(DbType::Json(Json).to_string(), "application/json");
    }

    #[test]
    fn db_type_xml_displays_as_xml_content_type() {
        assert_eq!(DbType::XML(Xml).to_string(), "application/xml");
    }

    #[test]
    fn metadata_type_all_displays_as_true() {
        assert_eq!(MetadataType::All.to_string(), "true");
    }

    #[test]
    fn metadata_type_key_displays_as_node_key() {
        assert_eq!(MetadataType::Key.to_string(), "nodeKey");
    }

    #[test]
    fn metadata_type_key_and_child_displays_correctly() {
        assert_eq!(
            MetadataType::KeyAndChild.to_string(),
            "nodeKeyAndChildCount"
        );
    }

    #[test]
    fn query_new_sets_all_fields() {
        let q = Query::new("sdb:diff".to_string(), Some(0), Some(10));
        assert_eq!(q.query, "sdb:diff");
        assert_eq!(q.start_result_seq_index, Some(0));
        assert_eq!(q.end_result_seq_index, Some(10));
    }

    #[test]
    fn query_new_without_optional_indices() {
        let q = Query::new("sdb:diff".to_string(), None, None);
        assert_eq!(q.query, "sdb:diff");
        assert_eq!(q.start_result_seq_index, None);
        assert_eq!(q.end_result_seq_index, None);
    }

    #[test]
    fn query_serializes_with_renamed_fields() {
        let q = Query::new("jn:doc('db','res')".to_string(), Some(0), Some(5));
        let json = serde_json::to_string(&q).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["query"], "jn:doc('db','res')");
        assert_eq!(v["startResultSeqIndex"], 0);
        assert_eq!(v["endResultSeqIndex"], 5);
    }

    #[test]
    fn query_serializes_none_indices_as_null() {
        let q = Query::new("test".to_string(), None, None);
        let json = serde_json::to_string(&q).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["query"], "test");
        assert!(v["startResultSeqIndex"].is_null());
        assert!(v["endResultSeqIndex"].is_null());
    }

    #[test]
    fn db_info_deserializes_with_resources() {
        let json = r#"{"resources": ["res1", "res2"]}"#;
        let db_info: DbInfo = serde_json::from_str(json).unwrap();
        assert_eq!(db_info.resources, vec!["res1", "res2"]);
    }

    #[test]
    fn db_info_deserializes_empty_resources() {
        let json = r#"{"resources": []}"#;
        let db_info: DbInfo = serde_json::from_str(json).unwrap();
        assert!(db_info.resources.is_empty());
    }

    #[test]
    fn info_result_deserializes_from_json() {
        let json = r#"{"name": "testdb", "type": "json"}"#;
        let result: InfoResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.name, "testdb");
        assert_eq!(result.info_type, "json");
    }

    #[test]
    fn info_results_with_resources_container_deserializes() {
        let json =
            r#"{"databases": [{"name": "db1", "type": "json", "resources": ["r1"]}]}"#;
        let result: InfoResultsWithResourcesContainer = serde_json::from_str(json).unwrap();
        assert_eq!(result.databases.len(), 1);
        assert_eq!(result.databases[0].name, "db1");
        assert_eq!(result.databases[0].resources, vec!["r1"]);
    }

    #[test]
    fn info_results_with_resources_container_deserializes_empty() {
        let json = r#"{"databases": []}"#;
        let result: InfoResultsWithResourcesContainer = serde_json::from_str(json).unwrap();
        assert!(result.databases.is_empty());
    }

    #[test]
    fn commit_deserializes_from_json() {
        let json = r#"{"revision_timestamp":"2021-01-01","revision":1,"author":"admin","commit_message":"initial"}"#;
        let commit: Commit = serde_json::from_str(json).unwrap();
        assert_eq!(commit.revision_timestamp, "2021-01-01");
        assert_eq!(commit.revision, 1);
        assert_eq!(commit.author, "admin");
        assert_eq!(commit.commit_message, "initial");
    }

    #[test]
    fn history_deserializes_from_json() {
        let json = r#"[{"revision_timestamp":"2021-01-01","revision":1,"author":"admin","commit_message":"initial"}]"#;
        let history: History = serde_json::from_str(json).unwrap();
        assert_eq!(history.0.len(), 1);
    }

    #[test]
    fn info_results_default_is_empty() {
        let results = InfoResults::default();
        assert!(results.0.is_empty());
    }

    #[test]
    fn info_results_with_resources_container_default_is_empty() {
        let results = InfoResultsWithResourcesContainer::default();
        assert!(results.databases.is_empty());
    }

    #[test]
    fn info_result_with_resources_deserializes() {
        let json = r#"{"name": "mydb", "type": "xml", "resources": ["a", "b"]}"#;
        let result: InfoResultWithResources = serde_json::from_str(json).unwrap();
        assert_eq!(result.name, "mydb");
        assert_eq!(result.info_type, "xml");
        assert_eq!(result.resources, vec!["a", "b"]);
    }
}
