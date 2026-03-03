//! This module contains types for holding token information and the various node types

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::{fmt, io, str::FromStr};

/// A specific connection token
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Serialize, std::cmp::PartialEq))]
pub struct TokenData {
    pub access_token: String,
    //pub expires_at: u64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_type_primitive_display_all_variants() {
        assert_eq!(NodeTypePrimitive::BooleanValue.to_string(), "BOOLEAN_VALUE");
        assert_eq!(NodeTypePrimitive::NullValue.to_string(), "NULL_VALUE");
        assert_eq!(NodeTypePrimitive::NumberValue.to_string(), "NUMBER_VALUE");
        assert_eq!(
            NodeTypePrimitive::ObjectBooleanValue.to_string(),
            "OBJECT_BOOLEAN_VALUE"
        );
        assert_eq!(NodeTypePrimitive::ObjectKey.to_string(), "OBJECT_KEY");
        assert_eq!(
            NodeTypePrimitive::ObjectNullValue.to_string(),
            "OBJECT_NULL_VALUE"
        );
        assert_eq!(
            NodeTypePrimitive::ObjectStringValue.to_string(),
            "OBJECT_STRING_VALUE"
        );
        assert_eq!(NodeTypePrimitive::StringValue.to_string(), "STRING_VALUE");
    }

    #[test]
    fn node_type_primitive_from_str_roundtrips() {
        let cases = vec![
            ("BOOLEAN_VALUE", "BOOLEAN_VALUE"),
            ("NULL_VALUE", "NULL_VALUE"),
            ("NUMBER_VALUE", "NUMBER_VALUE"),
            ("OBJECT_BOOLEAN_VALUE", "OBJECT_BOOLEAN_VALUE"),
            ("OBJECT_KEY", "OBJECT_KEY"),
            ("OBJECT_NULL_VALUE", "OBJECT_NULL_VALUE"),
            ("OBJECT_STRING_VALUE", "OBJECT_STRING_VALUE"),
            ("STRING_VALUE", "STRING_VALUE"),
        ];
        for (input, expected) in cases {
            let parsed: NodeTypePrimitive = input.parse().unwrap();
            assert_eq!(parsed.to_string(), expected);
        }
    }

    #[test]
    fn node_type_primitive_from_str_rejects_invalid() {
        let result: Result<NodeTypePrimitive, _> = "INVALID_TYPE".parse();
        assert!(result.is_err());
    }

    #[test]
    fn node_type_container_display_both_variants() {
        assert_eq!(NodeTypeContainer::Array.to_string(), "ARRAY");
        assert_eq!(NodeTypeContainer::Object.to_string(), "OBJECT");
    }

    #[test]
    fn node_type_container_from_str_roundtrips() {
        let array: NodeTypeContainer = "ARRAY".parse().unwrap();
        assert_eq!(array.to_string(), "ARRAY");
        let object: NodeTypeContainer = "OBJECT".parse().unwrap();
        assert_eq!(object.to_string(), "OBJECT");
    }

    #[test]
    fn node_type_container_from_str_rejects_invalid() {
        let result: Result<NodeTypeContainer, _> = "INVALID".parse();
        assert!(result.is_err());
    }

    #[test]
    fn token_data_deserializes_all_fields() {
        let json = r#"{
            "access_token": "abc123",
            "expires_in": 300,
            "not-before-policy": 0,
            "refresh_expires_in": 1800,
            "refresh_token": "refresh123",
            "scope": "profile email",
            "session_state": "sess123",
            "token_type": "bearer"
        }"#;
        let token: TokenData = serde_json::from_str(json).unwrap();
        assert_eq!(token.access_token, "abc123");
        assert_eq!(token.expires_in, 300);
        assert_eq!(token.not_before_policy, 0);
        assert_eq!(token.refresh_expires_in, 1800);
        assert_eq!(token.refresh_token, "refresh123");
        assert_eq!(token.scope, "profile email");
        assert_eq!(token.session_state, "sess123");
        assert_eq!(token.token_type, "bearer");
    }

    #[test]
    fn token_post_data_serializes_correctly() {
        let data = TokenPostData {
            username: "admin".to_string(),
            password: "pass".to_string(),
            grant_type: "password".to_string(),
        };
        let json = serde_json::to_string(&data).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["username"], "admin");
        assert_eq!(v["password"], "pass");
        assert_eq!(v["grant_type"], "password");
    }

    #[test]
    fn token_data_serialize_deserialize_roundtrip() {
        let json = r#"{
            "access_token": "tok",
            "expires_in": 300,
            "not-before-policy": 0,
            "refresh_expires_in": 1800,
            "refresh_token": "ref",
            "scope": "profile",
            "session_state": "sess",
            "token_type": "bearer"
        }"#;
        let token: TokenData = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&token).unwrap();
        let deserialized: TokenData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(token, deserialized);
    }
}
