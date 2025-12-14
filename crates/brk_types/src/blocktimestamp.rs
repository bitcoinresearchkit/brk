use schemars::JsonSchema;
use serde::Serialize;

use crate::{BlockHash, Height};

/// Block information returned for timestamp queries
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BlockTimestamp {
    /// Block height
    pub height: Height,

    /// Block hash
    pub hash: BlockHash,

    /// Block timestamp in ISO 8601 format
    pub timestamp: String,
}
