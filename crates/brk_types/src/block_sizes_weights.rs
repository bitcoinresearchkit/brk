use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{BlockSizeEntry, BlockWeightEntry};

/// Combined block sizes and weights response.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BlockSizesWeights {
    /// Block size data points
    pub sizes: Vec<BlockSizeEntry>,
    /// Block weight data points
    pub weights: Vec<BlockWeightEntry>,
}
