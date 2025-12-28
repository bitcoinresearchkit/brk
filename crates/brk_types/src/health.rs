use std::borrow::Cow;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Server health status
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    pub status: Cow<'static, str>,
    pub service: Cow<'static, str>,
    pub timestamp: String,
}
