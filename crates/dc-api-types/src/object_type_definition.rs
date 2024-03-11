/*
 *
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document:
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::GraphQLName;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectTypeDefinition {
    /// The columns of the type
    #[serde(rename = "columns")]
    pub columns: Vec<crate::ColumnInfo>,
    /// The description of the type
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the type
    #[serde(rename = "name")]
    pub name: GraphQLName,
}

impl ObjectTypeDefinition {
    pub fn new(columns: Vec<crate::ColumnInfo>, name: GraphQLName) -> ObjectTypeDefinition {
        ObjectTypeDefinition {
            columns,
            description: None,
            name,
        }
    }
}
