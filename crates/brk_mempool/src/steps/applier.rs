use brk_types::{Transaction, TxidPrefix};
use parking_lot::RwLock;

use crate::{
    State, TxEntry, TxRemoval,
    steps::preparer::{TxAddition, TxsPulled},
};

/// Applies a prepared diff to in-memory mempool state under one write
/// guard. Body proceeds: bury removed → publish added → evict.
pub struct Applier;

impl Applier {
    /// Returns true iff anything changed.
    pub fn apply(lock: &RwLock<State>, pulled: TxsPulled) -> bool {
        let TxsPulled { added, removed } = pulled;
        let has_changes = !added.is_empty() || !removed.is_empty();

        let mut state = lock.write();
        Self::bury_removals(&mut state, removed);
        Self::publish_additions(&mut state, added);
        state.graveyard.evict_old();

        has_changes
    }

    fn bury_removals(state: &mut State, removed: Vec<(TxidPrefix, TxRemoval)>) {
        for (prefix, reason) in removed {
            Self::bury_one(state, &prefix, reason);
        }
    }

    fn bury_one(state: &mut State, prefix: &TxidPrefix, reason: TxRemoval) {
        let Some(record) = state.txs.remove_by_prefix(prefix) else {
            return;
        };
        let txid = record.entry.txid;
        state.info.remove(&record.tx, record.entry.fee);
        state.addrs.remove_tx(&record.tx, &txid);
        state.outpoint_spends.remove_spends(&record.tx, *prefix);
        state.graveyard.bury(txid, record.tx, record.entry, reason);
    }

    fn publish_additions(state: &mut State, added: Vec<TxAddition>) {
        for addition in added {
            if let Some((tx, entry)) = Self::resolve_addition(state, addition) {
                Self::publish_one(state, tx, entry);
            }
        }
    }

    fn resolve_addition(state: &mut State, addition: TxAddition) -> Option<(Transaction, TxEntry)> {
        match addition {
            TxAddition::Fresh { tx, entry } => Some((tx, entry)),
            TxAddition::Revived { entry } => {
                let tomb = state.graveyard.exhume(&entry.txid)?;
                Some((tomb.tx, entry))
            }
        }
    }

    fn publish_one(state: &mut State, tx: Transaction, entry: TxEntry) {
        let prefix = entry.txid_prefix();
        state.info.add(&tx, entry.fee);
        state.addrs.add_tx(&tx, &entry.txid);
        state.outpoint_spends.insert_spends(&tx, prefix);
        state.txs.insert(tx, entry);
    }
}
