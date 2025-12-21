use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Server health status
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    pub status: &'static str,
    pub service: &'static str,
    pub timestamp: String,
}
