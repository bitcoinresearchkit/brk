use brk_types::{Transaction, TxidPrefix};
use parking_lot::RwLock;

use crate::{
    State, TxEntry, TxRemoval,
    steps::{
        preparer::{TxAddition, TxsPulled},
        rebuilder::{Rebuilder, Snapshot},
    },
};

/// Applies a prepared diff to in-memory mempool state under one write
/// guard. Body proceeds: bury removed → publish added → evict.
pub struct Applier;

impl Applier {
    /// `rebuilder` supplies the previous cycle's snapshot. Burial reads
    /// each tomb's `chunk_rate` from the snapshot (always-fresh,
    /// package-aware via local linearization). The fallback to
    /// `entry.fee_rate()` is unreachable in steady state - every burial
    /// target was alive at the previous tick, so the snapshot has it.
    pub fn apply(lock: &RwLock<State>, rebuilder: &Rebuilder, pulled: TxsPulled) {
        let TxsPulled { added, removed } = pulled;
        let mut state = lock.write();
        Self::bury_removals(&mut state, rebuilder, removed);
        Self::publish_additions(&mut state, added);
        state.graveyard.evict_old();
    }

    fn bury_removals(
        state: &mut State,
        rebuilder: &Rebuilder,
        removed: Vec<(TxidPrefix, TxRemoval)>,
    ) {
        let snapshot = rebuilder.snapshot();
        for (prefix, reason) in removed {
            Self::bury_one(state, &snapshot, &prefix, reason);
        }
    }

    fn bury_one(state: &mut State, snapshot: &Snapshot, prefix: &TxidPrefix, reason: TxRemoval) {
        let Some(record) = state.txs.remove_by_prefix(prefix) else {
            return;
        };
        let chunk_rate = snapshot
            .chunk_rate_for(prefix)
            .unwrap_or_else(|| record.entry.fee_rate());
        state.info.remove(&record.tx, record.entry.fee);
        state.addrs.remove_tx(&record.tx);
        state.outpoint_spends.remove_spends(&record.tx, *prefix);
        state
            .graveyard
            .bury(record.tx, record.entry, chunk_rate, reason);
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
        state.addrs.add_tx(&tx);
        state.outpoint_spends.insert_spends(&tx, prefix);
        state.txs.insert(tx, entry);
    }
}
