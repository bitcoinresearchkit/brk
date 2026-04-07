use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::PoolSlug;

/// Mining pool identification for a block
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockPool {
    /// Unique pool identifier
    #[schemars(example = 44)]
    pub id: u8,

    /// Pool name
    #[schemars(example = &"Foundry USA")]
    pub name: String,

    /// URL-friendly pool identifier
    pub slug: PoolSlug,

    /// Miner name tags found in coinbase scriptsig
    pub miner_names: Option<Vec<String>>,
}
