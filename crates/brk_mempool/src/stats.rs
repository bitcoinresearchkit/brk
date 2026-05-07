//! Owned snapshot of mempool in-memory counters for diagnostic display.

use crate::Mempool;

#[derive(Debug, Clone)]
pub struct MempoolStats {
    pub info_count: usize,
    pub tx_count: usize,
    pub unresolved_count: usize,
    pub addr_count: usize,
    pub entry_slot_count: usize,
    pub entry_active_count: usize,
    pub entry_free_count: usize,
    pub outpoint_spend_count: usize,
    pub graveyard_tombstone_count: usize,
    pub graveyard_order_count: usize,
}

impl From<&Mempool> for MempoolStats {
    fn from(mempool: &Mempool) -> Self {
        let state = mempool.state();
        let info = state.info.read();
        let txs = state.txs.read();
        let addrs = state.addrs.read();
        let entries = state.entries.read();
        let outpoint_spends = state.outpoint_spends.read();
        let graveyard = state.graveyard.read();
        Self {
            info_count: info.count,
            tx_count: txs.len(),
            unresolved_count: txs.unresolved().len(),
            addr_count: addrs.len(),
            entry_slot_count: entries.entries().len(),
            entry_active_count: entries.active_count(),
            entry_free_count: entries.free_slots_count(),
            outpoint_spend_count: outpoint_spends.len(),
            graveyard_tombstone_count: graveyard.tombstones_len(),
            graveyard_order_count: graveyard.order_len(),
        }
    }
}
