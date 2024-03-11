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
pub struct OpenApiDiscriminator {
    #[serde(rename = "mapping", skip_serializing_if = "Option::is_none")]
    pub mapping: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "propertyName")]
    pub property_name: String,
}

impl OpenApiDiscriminator {
    pub fn new(property_name: String) -> OpenApiDiscriminator {
        OpenApiDiscriminator {
            mapping: None,
            property_name,
        }
    }
}
