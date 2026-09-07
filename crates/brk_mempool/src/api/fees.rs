//! Fee reads: tier recommendations, projected-block stats, per-tx rates.

use brk_error::Result;
use brk_types::{BlockHash, FeeRate, RecommendedFees, Txid};

use crate::{Mempool, snapshot::BlockStats};

impl Mempool {
    pub fn fees(&self) -> Result<RecommendedFees> {
        let snapshot = self.snapshot();
        snapshot.ensure_projection()?;
        Ok(snapshot.fees.clone())
    }

    pub fn block_stats(&self) -> Result<Vec<BlockStats>> {
        let snapshot = self.snapshot();
        snapshot.ensure_projection()?;
        Ok(snapshot.block_stats.clone())
    }

    /// Effective fee rate from a completed matching graph/live publication,
    /// with the burial-time chunk rate as fallback for removed transactions.
    pub fn effective_fee_rate(&self, txid: &Txid, tip: &BlockHash) -> Result<Option<FeeRate>> {
        let snapshot = self.snapshot();
        let state = self.read();
        state.ensure_snapshot_at(tip, &snapshot)?;
        Ok(state
            .txs
            .entry(txid)
            .map(|entry| {
                snapshot
                    .chunk_rate_for(txid)
                    .unwrap_or_else(|| entry.fee_rate())
            })
            .or_else(|| state.graveyard.get(txid).map(|tomb| tomb.chunk_rate)))
    }
}

#[cfg(test)]
#[path = "../../tests/unit/api/fees.rs"]
mod tests;
