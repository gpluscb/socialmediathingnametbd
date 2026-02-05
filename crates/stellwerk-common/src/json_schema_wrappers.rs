use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use time::{OffsetDateTime, serde::rfc3339};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonSchemaOffsetDateTime(#[serde(with = "rfc3339")] pub OffsetDateTime);

impl JsonSchema for JsonSchemaOffsetDateTime {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("UtcDateTime")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "format": "date-time",
        })
    }
}
