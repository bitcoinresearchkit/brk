use brk_types::{CpfpClusterChunk, CpfpClusterTxIndex, FeeRate, Sats, VSize};
use smallvec::SmallVec;

use super::LocalIdx;

pub struct Chunk {
    /// Cluster-local positions of the txs in this chunk, in topological
    /// order (parents before children). Populated by `Cluster::new`.
    pub txs: SmallVec<[LocalIdx; 4]>,
    pub fee: Sats,
    pub vsize: VSize,
}

impl Chunk {
    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }
}

impl From<&Chunk> for CpfpClusterChunk {
    fn from(chunk: &Chunk) -> Self {
        Self {
            txs: chunk
                .txs
                .iter()
                .map(|&local| CpfpClusterTxIndex::from(local.inner()))
                .collect(),
            feerate: chunk.fee_rate(),
        }
    }
}
