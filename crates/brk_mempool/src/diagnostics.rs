//! Cycle-internal counters surfaced for observability and the
//! `examples/mempool.rs` driver. Captured under a single read guard
//! by `MempoolStats::from(&Mempool)`.

use crate::Mempool;

#[derive(Debug, Clone, Default)]
pub struct MempoolStats {
    pub txs: usize,
    pub unresolved: usize,
    pub addrs: usize,
    pub outpoint_spends: usize,
    pub graveyard_tombstones: usize,
    pub graveyard_order: usize,
    pub rebuilds: u64,
    pub skip_cleans: u64,
}

impl From<&Mempool> for MempoolStats {
    fn from(mempool: &Mempool) -> Self {
        let inner = mempool.read();
        let rebuilder = mempool.rebuilder();
        Self {
            txs: inner.txs.len(),
            unresolved: inner.txs.unresolved().len(),
            addrs: inner.addrs.len(),
            outpoint_spends: inner.outpoint_spends.len(),
            graveyard_tombstones: inner.graveyard.tombstones_len(),
            graveyard_order: inner.graveyard.order_len(),
            rebuilds: rebuilder.rebuild_count(),
            skip_cleans: rebuilder.skip_clean_count(),
        }
    }
}
