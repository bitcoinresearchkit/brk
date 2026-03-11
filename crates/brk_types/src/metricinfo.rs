use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Index;

/// Metadata about a metric
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MetricInfo {
    /// Available indexes
    pub indexes: Vec<Index>,
    /// Value type (e.g. "f32", "u64", "Sats")
    #[serde(rename = "type")]
    pub value_type: Cow<'static, str>,
}
