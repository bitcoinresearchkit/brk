//! Per-cycle event report returned by [`super::Mempool::tick`].

use std::{sync::Arc, time::Duration};

use brk_types::{AddrBytes, BlockHash, FeeRate, Height, MempoolInfo, Sats, Timestamp, Txid, VSize};

use crate::{Snapshot, TxRemoval};

/// One pull cycle's worth of changes. Produced by
/// [`super::Mempool::tick`] after fetch → prepare → apply → prevouts →
/// rebuild. The snapshot is always present (the rebuilder runs every
/// cycle); compare `next_block_hash` across cycles if you need to
/// detect whether the projection actually changed.
pub struct Cycle {
    pub added: Vec<TxAdded>,
    pub removed: Vec<TxRemoved>,
    /// Addresses that went from 0 → 1+ live mempool txs this cycle.
    /// Same-cycle enter-then-leave is collapsed (no event in either list).
    pub addr_enters: Vec<AddrBytes>,
    /// Addresses that went from 1+ → 0 live mempool txs this cycle.
    pub addr_leaves: Vec<AddrBytes>,
    /// Latest confirmed block. Compare to the prior cycle's `tip_hash`
    /// to detect a new block.
    pub tip_hash: BlockHash,
    pub tip_height: Height,
    pub info: MempoolInfo,
    pub snapshot: Arc<Snapshot>,
    pub took: Duration,
}

#[derive(Debug, Clone, Copy)]
pub struct TxAdded {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    pub fee_rate: FeeRate,
    pub first_seen: Timestamp,
    pub kind: AddedKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddedKind {
    /// First time we've seen this txid.
    Fresh,
    /// Re-entered the pool after a prior removal still in the graveyard.
    Revived,
}

#[derive(Debug, Clone, Copy)]
pub struct TxRemoved {
    pub txid: Txid,
    pub reason: TxRemoval,
    /// Package-effective rate at burial. Same value the tx reported
    /// while alive - RBF predecessors keep their package rate, not a
    /// misleading isolated fee/vsize.
    pub chunk_rate: FeeRate,
}
