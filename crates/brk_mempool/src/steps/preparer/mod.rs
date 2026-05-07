//! Turn `Fetched` raws into a typed diff for the Applier. Pure CPU,
//! holds a read guard on `State` for the cycle. New txs are
//! classified into three buckets:
//!
//! - **live** - already in `known`, skipped.
//! - **revivable** - in the graveyard, resurrected from the tombstone.
//! - **fresh** - decoded from `new_raws`, prevouts resolved against
//!   the live mempool only. Confirmed-parent prevouts land as
//!   `prevout: None` and are filled post-apply by the resolver passed
//!   to `Mempool::update_with`.
//!
//! Removals are inferred by cross-referencing inputs.

use brk_rpc::RawTx;
use brk_types::{MempoolEntryInfo, Txid, TxidPrefix};
use parking_lot::RwLock;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    State,
    stores::{TxGraveyard, TxStore},
};

mod tx_addition;
mod tx_entry;
mod tx_removal;
mod txs_pulled;

pub use tx_addition::TxAddition;
pub use tx_entry::TxEntry;
pub use tx_removal::TxRemoval;
pub use txs_pulled::TxsPulled;

pub struct Preparer;

impl Preparer {
    pub fn prepare(
        entries_info: Vec<MempoolEntryInfo>,
        new_raws: FxHashMap<Txid, RawTx>,
        lock: &RwLock<State>,
    ) -> TxsPulled {
        let state = lock.read();

        let live = Self::live_set(&entries_info);
        let added = Self::classify_additions(entries_info, new_raws, &state.txs, &state.graveyard);
        let removed = TxRemoval::classify(&live, &added, &state.txs);

        TxsPulled { added, removed }
    }

    fn live_set(entries_info: &[MempoolEntryInfo]) -> FxHashSet<TxidPrefix> {
        entries_info
            .iter()
            .map(|info| TxidPrefix::from(&info.txid))
            .collect()
    }

    fn classify_additions(
        entries_info: Vec<MempoolEntryInfo>,
        mut new_raws: FxHashMap<Txid, RawTx>,
        known: &TxStore,
        graveyard: &TxGraveyard,
    ) -> Vec<TxAddition> {
        entries_info
            .iter()
            .filter_map(|info| Self::classify(info, known, graveyard, &mut new_raws))
            .collect()
    }

    fn classify(
        info: &MempoolEntryInfo,
        known: &TxStore,
        graveyard: &TxGraveyard,
        new_raws: &mut FxHashMap<Txid, RawTx>,
    ) -> Option<TxAddition> {
        if known.contains(&info.txid) {
            return None;
        }
        if let Some(tomb) = graveyard.get(&info.txid) {
            return Some(TxAddition::revived(info, tomb));
        }
        let raw = new_raws.remove(&info.txid)?;
        Some(TxAddition::fresh(info, raw, known))
    }
}
