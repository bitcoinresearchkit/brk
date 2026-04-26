use brk_types::MempoolInfo;
use parking_lot::RwLock;

use super::{AddrTracker, EntryPool, TxGraveyard, TxStore};
use crate::steps::{applier::Applier, preparer::Pulled};

/// The five buckets making up live mempool state.
///
/// Each bucket has its own `RwLock` so readers of different buckets
/// don't contend with each other; the Applier takes all five write
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
    /// Apply a prepared diff to all five buckets atomically. Returns
    /// true iff the Applier observed any change. Same-cycle prevout
    /// resolution is a separate pipeline step run by the orchestrator.
    pub fn apply(&self, pulled: Pulled) -> bool {
        Applier::apply(
            pulled,
            &mut self.info.write(),
            &mut self.txs.write(),
            &mut self.addrs.write(),
            &mut self.entries.write(),
            &mut self.graveyard.write(),
        )
    }
}
