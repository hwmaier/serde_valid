use std::ops::Deref;

use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct JsonPointer(pub jsonschema::paths::JSONPointer);

impl Deref for JsonPointer {
    type Target = jsonschema::paths::JSONPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JsonSchema for JsonPointer {
    fn schema_name() -> String {
        "JsonPointer".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: None,
            ..Default::default()
        }
        .into()
    }
}
