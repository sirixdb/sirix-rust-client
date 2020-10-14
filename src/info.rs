//! This module contains types for holding token information and the various node types

use serde::{Deserialize, Serialize};
use std::{fmt, io, str::FromStr};

/// A specific connection token
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Serialize, std::cmp::PartialEq))]
pub struct TokenData {
    pub access_token: String,
    pub expires_at: usize,
    pub expires_in: usize,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: usize,
    pub refresh_expires_in: usize,
    pub refresh_token: String,
    pub scope: String,
    pub session_state: String,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPostData {
    pub username: String,
    pub password: String,
    pub grant_type: String,
}

// FIXME: This is super verbose, is there a better way?

/// All the various node types
#[derive(Debug)]
pub enum NodeType {
    Array,
    BooleanValue,
    NullValue,
    NumberValue,
    Object,
    ObjectBooleanValue,
    ObjectKey,
    ObjectNullValue,
    ObjectStringValue,
    StringValue,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NodeType::*;
        let upper_snake = match self {
            Array => "ARRAY",
            BooleanValue => "BOOLEAN_VALUE",
            NullValue => "NULL_VALUE",
            NumberValue => "NUMBER_VALUE",
            Object => "OBJECT",
            ObjectBooleanValue => "OBJECT_BOOLEAN_VALUE",
            ObjectKey => "OBJECT_KEY",
            ObjectNullValue => "OBJECT_NULL_VALUE",
            ObjectStringValue => "OBJECT_STRING_VALUE",
            StringValue => "STRING_VALUE",
        };
        write!(f, "{}", upper_snake)
    }
}

impl FromStr for NodeType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NodeType::*;
        match s {
            "ARRAY" => Ok(Array),
            "BOOLEAN_VALUE" => Ok(BooleanValue),
            "NULL_VALUE" => Ok(NullValue),
            "NUMBER_VALUE" => Ok(NumberValue),
            "OBJECT" => Ok(Object),
            "OBJECT_BOOLEAN_VALUE" => Ok(ObjectBooleanValue),
            "OBJECT_KEY" => Ok(ObjectKey),
            "OBJECT_NULL_VALUE" => Ok(ObjectNullValue),
            "OBJECT_STRING_VALUE" => Ok(ObjectStringValue),
            "STRING_VALUE" => Ok(StringValue),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported node type {}", s),
            )),
        }
    }
}
