/*
 *
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document:
 *
 * Generated by: https://openapi-generator.tech
 */

///
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
}

impl ToString for RelationshipType {
    fn to_string(&self) -> String {
        match self {
            Self::Object => String::from("object"),
            Self::Array => String::from("array"),
        }
    }
}

impl Default for RelationshipType {
    fn default() -> RelationshipType {
        Self::Object
    }
}
