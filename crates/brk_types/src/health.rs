use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::SyncStatus;

/// Server health status
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    pub status: Cow<'static, str>,
    pub service: Cow<'static, str>,
    pub version: Cow<'static, str>,
    pub timestamp: String,
    /// Server start time (ISO 8601)
    pub started_at: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    #[serde(flatten)]
    pub sync: SyncStatus,
}
