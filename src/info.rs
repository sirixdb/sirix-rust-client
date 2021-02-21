//! This module contains types for holding token information and the various node types

use serde::{de, Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fmt, io, str::FromStr};

/// A specific connection token
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Serialize, std::cmp::PartialEq))]
pub struct TokenData {
    pub access_token: String,
    pub expires_at: u64,
    pub expires_in: u64,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: u64,
    pub refresh_expires_in: u64,
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

/// All the various node types
#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    NodeTypeContainer,
    NodeTypePrimitive,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NodeTypeContainer {
    #[serde_as(as = "DisplayFromStr")]
    Object,
    #[serde_as(as = "DisplayFromStr")]
    Array,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NodeTypePrimitive {
    #[serde_as(as = "DisplayFromStr")]
    BooleanValue,
    #[serde_as(as = "DisplayFromStr")]
    NullValue,
    #[serde_as(as = "DisplayFromStr")]
    NumberValue,
    #[serde_as(as = "DisplayFromStr")]
    ObjectBooleanValue,
    #[serde_as(as = "DisplayFromStr")]
    ObjectKey,
    #[serde_as(as = "DisplayFromStr")]
    ObjectNullValue,
    #[serde_as(as = "DisplayFromStr")]
    ObjectStringValue,
    #[serde_as(as = "DisplayFromStr")]
    StringValue,
}

impl fmt::Display for NodeTypePrimitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NodeTypePrimitive::*;
        let upper_snake = match self {
            BooleanValue => "BOOLEAN_VALUE",
            NullValue => "NULL_VALUE",
            NumberValue => "NUMBER_VALUE",
            ObjectBooleanValue => "OBJECT_BOOLEAN_VALUE",
            ObjectKey => "OBJECT_KEY",
            ObjectNullValue => "OBJECT_NULL_VALUE",
            ObjectStringValue => "OBJECT_STRING_VALUE",
            StringValue => "STRING_VALUE",
        };
        write!(f, "{}", upper_snake)
    }
}

impl FromStr for NodeTypePrimitive {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NodeTypePrimitive::*;
        match s {
            "BOOLEAN_VALUE" => Ok(BooleanValue),
            "NULL_VALUE" => Ok(NullValue),
            "NUMBER_VALUE" => Ok(NumberValue),
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

impl fmt::Display for NodeTypeContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NodeTypeContainer::*;
        let upper_snake = match self {
            Array => "ARRAY",
            Object => "OBJECT",
        };
        write!(f, "{}", upper_snake)
    }
}

impl FromStr for NodeTypeContainer {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NodeTypeContainer::*;
        match s {
            "ARRAY" => Ok(Array),
            "OBJECT" => Ok(Object),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported node type {}", s),
            )),
        }
    }
}
