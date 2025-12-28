use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Hex-encoded string
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Hex(String);
