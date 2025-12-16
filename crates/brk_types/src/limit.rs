use std::borrow::Cow;

use derive_deref::Deref;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::Deserialize;

/// Maximum number of results to return. Defaults to 100 if not specified.
#[derive(Debug, Deref, Deserialize)]
#[serde(default = "default_search_limit")]
pub struct Limit(usize);

impl Limit {
    pub const MIN: Self = Self(1);
    pub const DEFAULT: Self = Self(100);
}

fn default_search_limit() -> Limit {
    Limit::DEFAULT
}

impl JsonSchema for Limit {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Limit")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "object",
            "properties": {
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return. Defaults to 100 if not specified.",
                    "default": 100,
                    "examples": [1, 10, 100, 1000, 10000, 100000]
                }
            }
        })
    }
}
