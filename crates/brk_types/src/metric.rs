use std::{borrow::Cow, fmt::Display};

use derive_deref::Deref;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::Deserialize;

/// Metric name
#[derive(Debug, Clone, Deref, Deserialize)]
#[serde(transparent)]
pub struct Metric(String);

impl From<String> for Metric {
    #[inline]
    fn from(metric: String) -> Self {
        Self(metric)
    }
}

impl From<&str> for Metric {
    #[inline]
    fn from(metric: &str) -> Self {
        Self(metric.to_owned())
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonSchema for Metric {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Metric")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "description": "Metric name",
            "examples": ["price_close", "market_cap", "realized_price"]
        })
    }
}
