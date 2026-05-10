//! Turn `Fetched` raws into a typed diff for the Applier. Pure CPU,
//! holds a read guard on `State` for the cycle. New entries are
//! classified into two buckets:
//!
//! - **revivable** - in the graveyard, resurrected from the tombstone.
//! - **fresh** - decoded from `new_raws`, prevouts resolved against
//!   the live mempool only. Confirmed-parent prevouts land as
//!   `prevout: None` and are filled post-apply by the resolver passed
//!   to `Mempool::update_with`.
//!
//! Existing entries are not re-classified - they keep their first-sight
//! state on the live store. Removals are inferred by cross-referencing
//! inputs against the full `live_txids` set from the cycle's pull.

use brk_types::{MempoolEntryInfo, Transaction, Txid, TxidPrefix, Vout};
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

type SpentBy = FxHashMap<(Txid, Vout), Txid>;

pub struct Preparer;

impl Preparer {
    pub fn prepare(
        live_txids: &[Txid],
        new_entries: Vec<MempoolEntryInfo>,
        new_txs: FxHashMap<Txid, bitcoin::Transaction>,
        lock: &RwLock<State>,
    ) -> TxsPulled {
        let state = lock.read();

        let live: FxHashSet<TxidPrefix> = live_txids.iter().map(TxidPrefix::from).collect();
        let added = Self::classify_additions(new_entries, new_txs, &state.txs, &state.graveyard);
        let removed = Self::classify_removals(&live, &added, &state.txs);

        TxsPulled { added, removed }
    }

    fn classify_additions(
        new_entries: Vec<MempoolEntryInfo>,
        mut new_txs: FxHashMap<Txid, bitcoin::Transaction>,
        known: &TxStore,
        graveyard: &TxGraveyard,
    ) -> Vec<TxAddition> {
        new_entries
            .iter()
            .filter_map(|info| Self::classify_addition(info, known, graveyard, &mut new_txs))
            .collect()
    }

    fn classify_addition(
        info: &MempoolEntryInfo,
        known: &TxStore,
        graveyard: &TxGraveyard,
        new_txs: &mut FxHashMap<Txid, bitcoin::Transaction>,
    ) -> Option<TxAddition> {
        if known.contains(&info.txid) {
            return None;
        }
        if let Some(tomb) = graveyard.get(&info.txid) {
            return Some(TxAddition::revived(info, tomb));
        }
        let tx = new_txs.remove(&info.txid)?;
        Some(TxAddition::fresh(info, tx, known))
    }

    /// One `(prefix, reason)` per known tx that's gone from the live set,
    /// in `known` iteration order.
    fn classify_removals(
        live: &FxHashSet<TxidPrefix>,
        added: &[TxAddition],
        known: &TxStore,
    ) -> Vec<(TxidPrefix, TxRemoval)> {
        let spent_by = Self::build_spent_by(added);
        known
            .records()
            .filter_map(|(prefix, record)| {
                if live.contains(prefix) {
                    return None;
                }
                Some((*prefix, Self::removal_reason(&record.tx, &spent_by)))
            })
            .collect()
    }

    fn removal_reason(tx: &Transaction, spent_by: &SpentBy) -> TxRemoval {
        tx.input
            .iter()
            .find_map(|i| spent_by.get(&(i.txid, i.vout)).copied())
            .map_or(TxRemoval::Vanished, |by| TxRemoval::Replaced { by })
    }

    /// Only `Fresh` additions carry tx input data. Revived txs were
    /// already in-pool, so they can't be new spenders of anything.
    fn build_spent_by(added: &[TxAddition]) -> SpentBy {
        let mut spent_by: SpentBy = FxHashMap::default();
        for addition in added {
            if let TxAddition::Fresh { tx, .. } = addition {
                for txin in &tx.input {
                    spent_by.insert((txin.txid, txin.vout), tx.txid);
                }
            }
        }
        spent_by
    }
}
