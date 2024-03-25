mod database;

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{WithName, WithNameRef};

pub use self::database::{Collection, ObjectField, ObjectType, Type};

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(default)]
    pub collections: BTreeMap<String, Collection>,
    #[serde(default)]
    pub object_types: BTreeMap<String, ObjectType>,
}

impl Schema {
    pub fn into_named_collections(self) -> impl Iterator<Item = WithName<Collection>> {
        self.collections
            .into_iter()
            .map(|(name, field)| WithName::named(name, field))
    }

    pub fn into_named_object_types(self) -> impl Iterator<Item = WithName<ObjectType>> {
        self.object_types
            .into_iter()
            .map(|(name, field)| WithName::named(name, field))
    }

    pub fn named_collections(&self) -> impl Iterator<Item = WithNameRef<'_, Collection>> {
        self.collections
            .iter()
            .map(|(name, field)| WithNameRef::named(name, field))
    }

    pub fn named_object_types(&self) -> impl Iterator<Item = WithNameRef<'_, ObjectType>> {
        self.object_types
            .iter()
            .map(|(name, field)| WithNameRef::named(name, field))
    }
}
