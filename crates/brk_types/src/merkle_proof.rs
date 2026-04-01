use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Height;

/// Merkle inclusion proof for a transaction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MerkleProof {
    pub block_height: Height,
    pub merkle: Vec<String>,
    pub pos: usize,
}
