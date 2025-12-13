use schemars::JsonSchema;
use serde::Serialize;

use crate::{BlockHash, Height};

/// Block status indicating whether block is in the best chain
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct BlockStatus {
    /// Whether this block is in the best chain
    pub in_best_chain: bool,

    /// Block height (only if in best chain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,

    /// Hash of the next block in the best chain (only if in best chain and not tip)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_best: Option<BlockHash>,
}

impl BlockStatus {
    pub fn in_best_chain(height: Height, next_best: Option<BlockHash>) -> Self {
        Self {
            in_best_chain: true,
            height: Some(height),
            next_best,
        }
    }

    pub fn not_in_best_chain() -> Self {
        Self {
            in_best_chain: false,
            height: None,
            next_best: None,
        }
    }
}
