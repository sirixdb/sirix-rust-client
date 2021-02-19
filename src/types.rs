//! The various types used in SirixDB transactions

use super::info::NodeType;
use serde::{Deserialize, Serialize};
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
    depth: usize,
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
#[derive(Debug, Default, Deserialize)]
pub struct InfoResultsWithResources(Vec<InfoResultWithResources>);

#[derive(Debug, Default, Serialize)]
pub struct Query {
    #[serde(rename = "startResultSeqIndex")]
    start_result_seq_index: Option<u128>,
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
    pub revision: RevisionArg,
    pub max_level: Option<u64>,
    pub top_level_limit: Option<u128>,
    pub top_level_skip_last_node: Option<u128>,
}

/// A diff from an insert operation
#[derive(Debug)]
pub struct InsertDiff {
    node_key: usize,
    insert_position_node_key: usize,
    insert_position: String,
    dewey_id: String,
    depth: usize,
    insert_type: String,
    data: String,
}

/// Transaction metadata
///
/// `descendant_count` and `child_count` are only provided if `node_type` is NodeType::Object or NodeType::Array
#[derive(Debug, Deserialize)]
pub struct Metadata {
    node_key: usize,
    hash: isize,
    node_type: NodeType,
    descendant_count: Option<usize>,
    child_count: Option<usize>,
}

/*
    class MetaNode(TypedDict):
        """
        ``key`` is provided only if ``type`` is :py:class:`pysirix.info.NodeType` ``OBJECT_KEY``.
        ``value`` is of type ``List[MetaNode]`` if ``metadata.type`` is ``OBJECT`` or ``ARRAY``,
        however, if ``metadata.childCount`` is 0, then ``value`` is an emtpy ``dict``, or an empty
        ``list``, depending on whether ``metadata.type`` is ``OBJECT`` or ``ARRAY``.
        ``value`` is of type :py:class:`MetaNode` if ``metadata.type`` is ``OBJECT_KEY``.
        ``value`` is a ``str`` if ``metadata.type`` is ``OBJECT_STRING_VALUE`` or ``STRING_VALUE``.
        ``value`` is an ``int`` or ``float`` if ``metadata.type`` == ``OBJECT_NUMBER_VALUE`` or ``NUMBER_VALUE``.
        ``value`` is a ``bool`` if ``metadata.type`` is ``OBJECT_BOOLEAN_VALUE`` or ``BOOLEAN_VALUE``.
        ``value`` is ``None`` if ``metadata.type`` is ``OBJECT_NULL_VALUE`` or ``NULL_VALUE``.
        """

        metadata: Metadata
        key: str
        value: Union[
            List[Iterable["MetaNode"]],
            Iterable["MetaNode"],
            str,
            int,
            float,
            bool,
            None,
        ]
*/

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
pub struct XML;
#[derive(Debug, Clone)]
pub struct Json;

/// All possible options database (and resource) types
#[derive(Debug, Clone)]
pub enum DbType {
    Json(Json),
    XML(XML),
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
