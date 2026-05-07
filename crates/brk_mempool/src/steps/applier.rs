use brk_types::{Transaction, Txid, TxidPrefix};
use tracing::warn;

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

    fn bury_one(s: &mut LockedState, prefix: &TxidPrefix, reason: TxRemoval) {
        let Some(txid) = s.entries.get(prefix).map(|e| e.txid) else {
            return;
        };
        if !s.txs.contains(&txid) {
            // entries had this prefix but txs didn't — a state divergence
            // that should be impossible: publish/bury both touch them
            // together under one write_all guard. Reaching this branch
            // means a prior cycle left the two stores out of sync (e.g.
            // panic mid-publish before `txs.extend` ran). Skip the bury
            // entirely: freeing the entries slot here would let
            // outpoint_spends point at a slot the next insert recycles
            // for an unrelated tx.
            warn!(
                "mempool bury: entry present but tx missing for txid={txid} - skipping bury to preserve outpoint_spends integrity"
            );
            return;
        }
        let (idx, entry) = s.entries.remove(prefix).expect("entry present");
        let tx = s.txs.remove(&txid).expect("tx present");
        s.info.remove(&tx, entry.fee);
        s.addrs.remove_tx(&tx, &txid);
        s.outpoint_spends.remove_spends(&tx, idx);
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

    fn publish_one(s: &mut LockedState, tx: Transaction, entry: TxEntry) -> (Txid, Transaction) {
        s.info.add(&tx, entry.fee);
        s.addrs.add_tx(&tx, &entry.txid);
        let txid = entry.txid;
        let idx = s.entries.insert(entry);
        s.outpoint_spends.insert_spends(&tx, idx);
        (txid, tx)
    }
}
