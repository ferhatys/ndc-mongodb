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
pub struct RawResponse {
    /// The rows returned by the raw query.
    #[serde(rename = "rows")]
    pub rows: Vec<
        ::std::collections::HashMap<String, ::std::collections::HashMap<String, serde_json::Value>>,
    >,
}

impl RawResponse {
    pub fn new(
        rows: Vec<
            ::std::collections::HashMap<
                String,
                ::std::collections::HashMap<String, serde_json::Value>,
            >,
        >,
    ) -> RawResponse {
        RawResponse { rows }
    }
}
