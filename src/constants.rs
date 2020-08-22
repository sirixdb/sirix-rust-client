//! Some hardcoded type unions for transactions

use std::fmt;

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

/// All possible options database (and resource) types
#[derive(Debug)]
pub enum DbType {
    Json,
    XML,
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DbType::*;
        let db_type = match self {
            Json => "application/json",
            XML => "application/xml",
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

/// A wrapper for a revision (int, datetime)
// TODO: chrono??
pub struct Revision(usize, usize);
