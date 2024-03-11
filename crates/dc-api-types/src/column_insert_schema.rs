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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ColumnInsertSchema {
    /// The name of the column that this field should be inserted into
    #[serde(rename = "column")]
    pub column: String,
    #[serde(rename = "column_type")]
    pub column_type: String,
    /// Is the column nullable
    #[serde(rename = "nullable")]
    pub nullable: bool,
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "value_generated", skip_serializing_if = "Option::is_none")]
    pub value_generated: Option<Box<crate::ColumnValueGenerationStrategy>>,
}

impl ColumnInsertSchema {
    pub fn new(
        column: String,
        column_type: String,
        nullable: bool,
        r#type: RHashType,
    ) -> ColumnInsertSchema {
        ColumnInsertSchema {
            column,
            column_type,
            nullable,
            r#type,
            value_generated: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "column")]
    Column,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Column
    }
}
