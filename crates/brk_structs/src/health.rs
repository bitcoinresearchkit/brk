use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Server health status
pub struct Health {
    pub status: &'static str,
    pub service: &'static str,
    pub timestamp: String,
}
