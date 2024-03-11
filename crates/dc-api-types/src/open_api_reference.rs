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
pub struct OpenApiReference {
    #[serde(rename = "$ref")]
    pub dollar_ref: String,
}

impl OpenApiReference {
    pub fn new(dollar_ref: String) -> OpenApiReference {
        OpenApiReference { dollar_ref }
    }
}
