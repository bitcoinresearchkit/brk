use schemars::JsonSchema;
use serde::Serialize;

use super::{BlockSizeEntry, BlockWeightEntry};

/// Combined block sizes and weights response.
#[derive(Debug, Serialize, JsonSchema)]
pub struct BlockSizesWeights {
    pub sizes: Vec<BlockSizeEntry>,
    pub weights: Vec<BlockWeightEntry>,
}
