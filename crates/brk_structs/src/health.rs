use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Server health status
pub struct Health {
    pub status: String,
    pub service: String,
    pub timestamp: String,
}
