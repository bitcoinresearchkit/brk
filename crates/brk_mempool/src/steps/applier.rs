use brk_types::{Transaction, Txid, TxidPrefix};

use crate::{
    TxEntry, TxRemoval,
    steps::preparer::{TxAddition, TxsPulled},
    stores::{LockedState, MempoolState},
};

/// Applies a prepared diff to in-memory mempool state. All five write
/// locks are taken in canonical order via `MempoolState::write_all`,
/// then the body proceeds as: bury removed → publish added → evict.
pub struct Applier;

impl Applier {
    /// Returns true iff anything changed.
    pub fn apply(state: &MempoolState, pulled: TxsPulled) -> bool {
        let TxsPulled { added, removed } = pulled;
        let has_changes = !added.is_empty() || !removed.is_empty();

        let mut s = state.write_all();
        Self::bury_removals(&mut s, removed);
        Self::publish_additions(&mut s, added);
        s.graveyard.evict_old();

        has_changes
    }

    fn bury_removals(s: &mut LockedState, removed: Vec<(TxidPrefix, TxRemoval)>) {
        for (prefix, reason) in removed {
            Self::bury_one(s, &prefix, reason);
        }
    }

    /// Move one tx from the live mempool into the graveyard. Removes
    /// from every store + tracker, then hands the body to
    /// `graveyard.bury`. Silently bails if the entry or tx body is
    /// already gone (idempotent under repeated removals).
    fn bury_one(s: &mut LockedState, prefix: &TxidPrefix, reason: TxRemoval) {
        let Some(entry) = s.entries.remove(prefix) else {
            return;
        };
        let txid = entry.txid.clone();
        let Some(tx) = s.txs.remove(&txid) else {
            return;
        };
        s.info.remove(&tx, entry.fee);
        s.addrs.remove_tx(&tx, &txid);
        s.graveyard.bury(txid, tx, entry, reason);
    }

    fn publish_additions(s: &mut LockedState, added: Vec<TxAddition>) {
        let mut to_store: Vec<(Txid, Transaction)> = Vec::with_capacity(added.len());
        for addition in added {
            if let Some((tx, entry)) = Self::resolve_addition(s, addition) {
                to_store.push(Self::publish_one(s, tx, entry));
            }
        }
        s.txs.extend(to_store);
    }

    /// Materialize a `TxAddition` into the (tx, entry) pair the Applier
    /// will publish. Fresh additions are already-decoded; Revived ones
    /// pull the cached body out of the graveyard and skip if it's gone.
    fn resolve_addition(
        s: &mut LockedState,
        addition: TxAddition,
    ) -> Option<(Transaction, TxEntry)> {
        match addition {
            TxAddition::Fresh { tx, entry } => Some((tx, entry)),
            TxAddition::Revived { entry } => {
                let tomb = s.graveyard.exhume(&entry.txid)?;
                Some((tomb.tx, entry))
            }
        }
    }

    /// Publish one tx into the live mempool: fold its fee into info,
    /// register addr deltas, store the entry. Returns `(txid, tx)` for
    /// the caller to batch into `txs.extend` once at the end.
    fn publish_one(s: &mut LockedState, tx: Transaction, entry: TxEntry) -> (Txid, Transaction) {
        s.info.add(&tx, entry.fee);
        s.addrs.add_tx(&tx, &entry.txid);
        let txid = entry.txid.clone();
        s.entries.insert(entry);
        (txid, tx)
    }
}
