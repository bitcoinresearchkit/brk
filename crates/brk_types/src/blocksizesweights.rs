use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{BlockSizeEntry, BlockWeightEntry};

/// Combined block sizes and weights response.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BlockSizesWeights {
    pub sizes: Vec<BlockSizeEntry>,
    pub weights: Vec<BlockWeightEntry>,
}
