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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Aggregate {
    #[serde(rename = "column_count")]
    ColumnCount {
        /// The column to apply the count aggregate function to
        #[serde(rename = "column")]
        column: String,
        /// Whether or not only distinct items should be counted
        #[serde(rename = "distinct")]
        distinct: bool,
    },
    #[serde(rename = "single_column")]
    SingleColumn {
        /// The column to apply the aggregation function to
        #[serde(rename = "column")]
        column: String,
        /// Single column aggregate function name. A valid GraphQL name
        #[serde(rename = "function")]
        function: String,
        #[serde(rename = "result_type")]
        result_type: String,
    },
    #[serde(rename = "star_count")]
    StarCount {},
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "star_count")]
    StarCount,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::StarCount
    }
}
