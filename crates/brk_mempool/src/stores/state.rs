use brk_types::MempoolInfo;
use parking_lot::{RwLock, RwLockWriteGuard};

use super::{AddrTracker, EntryPool, OutpointSpends, TxGraveyard, TxStore};

/// The six buckets making up live mempool state.
///
/// Each bucket has its own `RwLock` so readers of different buckets
/// don't contend with each other. Any code that takes more than one
/// lock must follow the canonical partial order
/// `info → txs → addrs → entries → outpoint_spends → graveyard`,
/// otherwise a reader-holds-A-wants-B / writer-holds-B-wants-A
/// circular wait can deadlock. The Applier takes all six write locks
/// in this order for a brief window once per cycle via
/// [`MempoolState::write_all`]; multi-lock readers inside the crate
/// take a (canonical-order) subset inline.
///
/// This discipline is *internal* to `brk_mempool`: external crates
/// only see `Mempool` methods that bundle each multi-lock operation
/// behind a single call (e.g. `Mempool::lookup_spender`,
/// `Mempool::addr_txs`, `Mempool::rbf_for_tx`), so callers can never
/// take the order wrong because they don't get to choose.
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
