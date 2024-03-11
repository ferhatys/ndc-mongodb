/*
 *
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document:
 *
 * Generated by: https://openapi-generator.tech
 */

use ::std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryResponse {
    /// In a foreach query we respond with multiple result sets, one for each foreach predicate.
    /// This variant uses a struct constructor to reflect the API JSON format.
    ForEach { rows: Vec<ForEachRow> },
    /// In a non-foreach query we respond with a single result set.
    /// This variant uses a tuple constructor to reflect the lack of a wrapping object in the API
    /// JSON format.
    Single(RowSet),
}

impl QueryResponse {
    pub fn new() -> QueryResponse {
        QueryResponse::Single(Default::default())
    }
}

impl Default for QueryResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForEachRow {
    pub query: RowSet,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RowSet {
    /// The results of the aggregates returned by the query
    pub aggregates: Option<HashMap<String, serde_json::Value>>,
    /// The rows returned by the query, corresponding to the query's fields
    pub rows: Option<Vec<HashMap<String, ResponseFieldValue>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFieldValue {
    Relationship(Box<RowSet>),
    Column(serde_json::Value),
}
