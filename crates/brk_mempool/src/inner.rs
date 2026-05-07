//! Single-locked container for the live mempool.
//!
//! All cycle steps and read-side accessors take a guard on this one
//! lock. The substructures are plain owned types — they used to each
//! own a RwLock, but the canonical lock-order discipline disappears
//! when there's nothing to order.

use brk_types::MempoolInfo;

use crate::stores::{AddrTracker, OutpointSpends, TxGraveyard, TxStore};

#[derive(Default)]
pub struct MempoolInner {
    pub info: MempoolInfo,
    pub txs: TxStore,
    pub addrs: AddrTracker,
    pub outpoint_spends: OutpointSpends,
    pub graveyard: TxGraveyard,
}
