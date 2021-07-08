//! The various types used in SirixDB transactions

use super::info::{NodeTypeContainer, NodeTypePrimitive};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// A single commit
#[derive(Debug, Deserialize)]
pub struct Commit {
    revision_timestamp: String,
    revision: usize,
    author: String,
    commit_message: String,
}

#[derive(Debug, Deserialize)]
pub struct History(Vec<Commit>);

type Resources = Vec<String>;

/// database info struct
#[derive(Debug, Deserialize)]
pub struct DbInfo {
    resources: Resources, // should this be full-on resources?
}

/// A diff from a delete operation
#[derive(Debug)]
pub struct DeleteDiff {
    node_key: usize,
    dewey_id: String,
    depth: u64,
}

/// A result from the global info request
#[derive(Debug, Deserialize)]
pub struct InfoResult {
    name: String,
    info_type: String,
}

/// A result from the global info request, resources included
#[derive(Debug, Deserialize)]
pub struct InfoResultWithResources {
    name: String,
    info_type: String,
    resources: Resources,
}

/// A full list for the global info request, without resources
#[derive(Debug, Default, Deserialize)]
pub struct InfoResults(Vec<InfoResult>);

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
    #[serde(rename = "startResultSeqIndex")]
    start_result_seq_index: Option<u128>,
    #[serde(rename = "endResultSeqIndex")]
    end_result_seq_index: Option<u128>,
    query: String,
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
    node_key: u128,
    insert_position_node_key: usize,
    insert_position: String,
    dewey_id: String,
    depth: u64,
    insert_type: String,
    data: String,
}

#[derive(Debug, Deserialize)]
pub struct MetadataPrimitive {
    node_key: usize,
    hash: isize,
    node_type: NodeTypePrimitive,
}

#[derive(Debug, Deserialize)]
pub struct MetadataContainer {
    node_key: usize,
    hash: isize,
    node_type: NodeTypeContainer,
    descendant_count: usize,
    child_count: usize,
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
    revision_number: usize,
    revision_timestamp: String,
    revision: Revision,
}

/// A diff from a replace operation
#[derive(Debug)]
pub struct ReplaceDiff {
    node_key: usize,
    replace_type: String,
    data: String,
}

/// A timestamped revision ID
#[derive(Debug)]
pub struct Revision {
    timestamp: String,
    revision: usize,
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
