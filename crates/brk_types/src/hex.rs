use schemars::JsonSchema;
use serde::Serialize;

/// Hex-encoded string
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(transparent)]
pub struct Hex(String);
