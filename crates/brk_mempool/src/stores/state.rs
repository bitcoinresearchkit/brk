use brk_types::MempoolInfo;
use parking_lot::{RwLock, RwLockWriteGuard};

use super::{AddrTracker, EntryPool, TxGraveyard, TxStore};

/// The five buckets making up live mempool state.
///
/// Each bucket has its own `RwLock` so readers of different buckets
/// don't contend with each other. The Applier takes all five write
/// locks in a fixed order for a brief window once per cycle.
#[derive(Default)]
pub struct MempoolState {
    pub(crate) info: RwLock<MempoolInfo>,
    pub(crate) txs: RwLock<TxStore>,
    pub(crate) addrs: RwLock<AddrTracker>,
    pub(crate) entries: RwLock<EntryPool>,
    pub(crate) graveyard: RwLock<TxGraveyard>,
}

impl MempoolState {
    /// All five write guards in the canonical lock order. Used by the
    /// Applier to apply a sync diff atomically.
    pub(crate) fn write_all(&self) -> LockedState<'_> {
        LockedState {
            info: self.info.write(),
            txs: self.txs.write(),
            addrs: self.addrs.write(),
            entries: self.entries.write(),
            graveyard: self.graveyard.write(),
        }
    }
}

pub(crate) struct LockedState<'a> {
    pub info: RwLockWriteGuard<'a, MempoolInfo>,
    pub txs: RwLockWriteGuard<'a, TxStore>,
    pub addrs: RwLockWriteGuard<'a, AddrTracker>,
    pub entries: RwLockWriteGuard<'a, EntryPool>,
    pub graveyard: RwLockWriteGuard<'a, TxGraveyard>,
}
