use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::FeeRate;

use super::CpfpClusterTxIndex;

/// One SFL chunk inside a `CpfpCluster`. `txs` is in topological order
/// (matches `CpfpCluster.txs` ordering); the chunk's `feerate` is the
/// per-chunk SFL feerate and is the same for every tx in this chunk.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CpfpClusterChunk {
    pub txs: Vec<CpfpClusterTxIndex>,
    pub feerate: FeeRate,
}

/// Find the chunk containing `seed_local` and return `(chunk_index,
/// feerate)`. Falls back to `(0, fallback)` when the seed isn't in any
/// chunk - shouldn't happen for a well-formed linearization but keeps
/// callers' wire shape valid.
pub fn find_seed_chunk(
    chunks: &[CpfpClusterChunk],
    seed_local: CpfpClusterTxIndex,
    fallback: FeeRate,
) -> (u32, FeeRate) {
    chunks
        .iter()
        .enumerate()
        .find(|(_, ch)| ch.txs.contains(&seed_local))
        .map(|(i, ch)| (i as u32, ch.feerate))
        .unwrap_or((0, fallback))
}
