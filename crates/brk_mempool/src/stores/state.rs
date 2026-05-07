use brk_types::MempoolInfo;
use parking_lot::{RwLock, RwLockWriteGuard};

use super::{AddrTracker, EntryPool, OutpointSpends, TxGraveyard, TxStore};

/// The six buckets making up live mempool state. Each has its own
/// `RwLock`. Multi-lock code must follow the canonical order
/// `info → txs → addrs → entries → outpoint_spends → graveyard` to
/// avoid circular waits. External callers go through bundled
/// `Mempool` methods so they can't take the order wrong.
#[derive(Default)]
pub struct MempoolState {
    pub(crate) info: RwLock<MempoolInfo>,
    pub(crate) txs: RwLock<TxStore>,
    pub(crate) addrs: RwLock<AddrTracker>,
    pub(crate) entries: RwLock<EntryPool>,
    pub outpoint_spends: RwLock<OutpointSpends>,
    pub(crate) graveyard: RwLock<TxGraveyard>,
}

impl MempoolState {
    /// All six write guards in the canonical lock order. Used by the
    /// Applier to apply a sync diff atomically.
    pub(crate) fn write_all(&self) -> LockedState<'_> {
        LockedState {
            info: self.info.write(),
            txs: self.txs.write(),
            addrs: self.addrs.write(),
            entries: self.entries.write(),
            outpoint_spends: self.outpoint_spends.write(),
            graveyard: self.graveyard.write(),
        }
    }
}

pub(crate) struct LockedState<'a> {
    pub info: RwLockWriteGuard<'a, MempoolInfo>,
    pub txs: RwLockWriteGuard<'a, TxStore>,
    pub addrs: RwLockWriteGuard<'a, AddrTracker>,
    pub entries: RwLockWriteGuard<'a, EntryPool>,
    pub outpoint_spends: RwLockWriteGuard<'a, OutpointSpends>,
    pub graveyard: RwLockWriteGuard<'a, TxGraveyard>,
}
