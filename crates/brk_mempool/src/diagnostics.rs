//! Writer progress counters copied into completed read publications.

use crate::Mempool;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MempoolStats {
    pub txs: usize,
    pub unresolved: usize,
    pub addrs: usize,
    pub outpoint_spends: usize,
    pub graveyard_tombstones: usize,
    pub graveyard_order: usize,
    pub rebuilds: u64,
}

impl From<&Mempool> for MempoolStats {
    fn from(mempool: &Mempool) -> Self {
        let state = &mempool.state;
        let rebuilder = &mempool.rebuilder;
        Self {
            txs: state.txs.len(),
            unresolved: state.txs.unresolved().len(),
            addrs: state.addrs.len(),
            outpoint_spends: state.outpoint_spends.len(),
            graveyard_tombstones: state.graveyard.tombstones_len(),
            graveyard_order: state.graveyard.order_len(),
            rebuilds: rebuilder.rebuild_count(),
        }
    }
}
