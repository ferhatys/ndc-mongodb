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
pub struct ConfigSchemaResponse {
    #[serde(rename = "config_schema")]
    pub config_schema: Box<crate::OpenApiSchema>,
    #[serde(rename = "other_schemas")]
    pub other_schemas: ::std::collections::HashMap<String, crate::OpenApiSchema>,
}

impl ConfigSchemaResponse {
    pub fn new(
        config_schema: crate::OpenApiSchema,
        other_schemas: ::std::collections::HashMap<String, crate::OpenApiSchema>,
    ) -> ConfigSchemaResponse {
        ConfigSchemaResponse {
            config_schema: Box::new(config_schema),
            other_schemas,
        }
    }
}
