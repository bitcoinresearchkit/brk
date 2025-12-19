use schemars::JsonSchema;
use serde::Serialize;

/// Server health status
#[derive(Debug, Serialize, JsonSchema)]
pub struct Health {
    pub status: &'static str,
    pub service: &'static str,
    pub timestamp: String,
}
