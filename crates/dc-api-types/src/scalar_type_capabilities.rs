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

/// ScalarTypeCapabilities : Capabilities of a scalar type. comparison_operators: The comparison operators supported by the scalar type. aggregate_functions: The aggregate functions supported by the scalar type. update_column_operators: The update column operators supported by the scalar type. graphql_type: Associates the custom scalar type with one of the built-in GraphQL scalar types.  If a `graphql_type` is specified then HGE will use the parser for that built-in type when parsing values of the custom type. If not given then any JSON value will be accepted.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalarTypeCapabilities {
    /// A map from aggregate function names to their result types. Function and result type names must be valid GraphQL names. Result type names must be defined scalar types declared in ScalarTypesCapabilities.
    #[serde(
        rename = "aggregate_functions",
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregate_functions: Option<::std::collections::HashMap<String, String>>,
    /// A map from comparison operator names to their argument types. Operator and argument type names must be valid GraphQL names. Argument type names must be defined scalar types declared in ScalarTypesCapabilities.
    #[serde(
        rename = "comparison_operators",
        skip_serializing_if = "Option::is_none"
    )]
    pub comparison_operators: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "graphql_type", skip_serializing_if = "Option::is_none")]
    pub graphql_type: Option<crate::GraphQlType>,
    /// A map from update column operator names to their definitions. Operator names must be valid GraphQL names.
    #[serde(
        rename = "update_column_operators",
        skip_serializing_if = "Option::is_none"
    )]
    pub update_column_operators:
        Option<::std::collections::HashMap<String, crate::UpdateColumnOperatorDefinition>>,
}

impl ScalarTypeCapabilities {
    /// Capabilities of a scalar type. comparison_operators: The comparison operators supported by the scalar type. aggregate_functions: The aggregate functions supported by the scalar type. update_column_operators: The update column operators supported by the scalar type. graphql_type: Associates the custom scalar type with one of the built-in GraphQL scalar types.  If a `graphql_type` is specified then HGE will use the parser for that built-in type when parsing values of the custom type. If not given then any JSON value will be accepted.
    pub fn new() -> ScalarTypeCapabilities {
        ScalarTypeCapabilities {
            aggregate_functions: None,
            comparison_operators: None,
            graphql_type: None,
            update_column_operators: None,
        }
    }
}
