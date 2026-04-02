use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Height;

/// Merkle inclusion proof for a transaction
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MerkleProof {
    /// Block height containing the transaction
    pub block_height: Height,
    /// Merkle proof path (hex-encoded hashes)
    pub merkle: Vec<String>,
    /// Transaction position in the block
    pub pos: usize,
}
