//! Turn `Fetched` raws into a typed diff for the Applier. Pure CPU,
//! holds read locks on `txs` and `graveyard` for the cycle. New txs
//! are classified into three buckets:
//!
//! - **live** - already in `known`, skipped.
//! - **revivable** - in the graveyard, resurrected from the tombstone.
//! - **fresh** - decoded from `new_raws`, prevouts resolved against
//!   `known` or `parent_raws`.
//!
//! Removals are inferred by cross-referencing inputs.

use brk_rpc::RawTx;
use brk_types::{MempoolEntryInfo, Txid, TxidPrefix};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
    steps::fetcher::Fetched,
    stores::{MempoolState, TxGraveyard, TxStore},
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
    pub fn prepare(fetched: Fetched, state: &MempoolState) -> TxsPulled {
        let known = state.txs.read();
        let graveyard = state.graveyard.read();

        let live = Self::live_set(&fetched.entries_info);
        let added = Self::classify_additions(fetched, &known, &graveyard);
        let removed = TxRemoval::classify(&live, &added, &known);

        TxsPulled { added, removed }
    }

    fn live_set(entries_info: &[MempoolEntryInfo]) -> FxHashSet<TxidPrefix> {
        entries_info.iter().map(|info| TxidPrefix::from(&info.txid)).collect()
    }

    fn classify_additions(
        fetched: Fetched,
        known: &TxStore,
        graveyard: &TxGraveyard,
    ) -> Vec<TxAddition> {
        let Fetched {
            entries_info,
            mut new_raws,
            parent_raws,
        } = fetched;

        entries_info
            .iter()
            .filter_map(|info| {
                Self::classify(info, known, graveyard, &mut new_raws, &parent_raws)
            })
            .collect()
    }

    fn classify(
        info: &MempoolEntryInfo,
        known: &TxStore,
        graveyard: &TxGraveyard,
        new_raws: &mut FxHashMap<Txid, RawTx>,
        parent_raws: &FxHashMap<Txid, RawTx>,
    ) -> Option<TxAddition> {
        if known.contains(&info.txid) {
            return None;
        }
        if let Some(tomb) = graveyard.get(&info.txid) {
            return Some(TxAddition::revived(info, tomb));
        }
        let raw = new_raws.remove(&info.txid)?;
        Some(TxAddition::fresh(info, raw, parent_raws, known))
    }
}
