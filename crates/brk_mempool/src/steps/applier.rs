use brk_types::{Transaction, TxidPrefix};
use parking_lot::RwLock;

use crate::{
    AddrTransitions, CycleDiff, State, TxEntry, TxRemoval,
    cycle::{TxAdded, TxRemoved},
    steps::{
        preparer::{TxAddition, TxsPulled},
        rebuilder::{Rebuilder, Snapshot},
    },
};

/// Applies a prepared diff to in-memory mempool state under one write
/// guard. Body proceeds: bury removed → publish added → evict. Events
/// are pushed into the caller-supplied [`CycleDiff`] accumulator.
pub struct Applier;

impl Applier {
    /// `rebuilder` supplies the previous cycle's snapshot. Burial reads
    /// each tomb's `chunk_rate` from the snapshot (always-fresh,
    /// package-aware via local linearization). The fallback to
    /// `entry.fee_rate()` is unreachable in steady state - every burial
    /// target was alive at the previous tick, so the snapshot has it.
    pub fn apply(
        lock: &RwLock<State>,
        rebuilder: &Rebuilder,
        pulled: TxsPulled,
        diff: &mut CycleDiff,
    ) {
        let TxsPulled { added, removed } = pulled;
        let mut state = lock.write();
        Self::bury_removals(&mut state, rebuilder, &mut diff.addrs, &mut diff.removed, removed);
        Self::publish_additions(&mut state, &mut diff.addrs, &mut diff.added, added);
        state.graveyard.evict_old();
    }

    fn bury_removals(
        state: &mut State,
        rebuilder: &Rebuilder,
        transitions: &mut AddrTransitions,
        events: &mut Vec<TxRemoved>,
        removed: Vec<(TxidPrefix, TxRemoval)>,
    ) {
        let snapshot = rebuilder.snapshot();
        events.reserve(removed.len());
        for (prefix, reason) in removed {
            if let Some(ev) = Self::bury_one(state, &snapshot, transitions, &prefix, reason) {
                events.push(ev);
            }
        }
    }

    fn bury_one(
        state: &mut State,
        snapshot: &Snapshot,
        transitions: &mut AddrTransitions,
        prefix: &TxidPrefix,
        reason: TxRemoval,
    ) -> Option<TxRemoved> {
        let record = state.txs.remove_by_prefix(prefix)?;
        let chunk_rate = snapshot
            .chunk_rate_for(prefix)
            .unwrap_or_else(|| record.entry.fee_rate());
        let txid = record.entry.txid;
        state.info.remove(&record.tx, record.entry.fee);
        state.addrs.remove_tx(transitions, &record.tx);
        state.outpoint_spends.remove_spends(&record.tx, *prefix);
        state
            .graveyard
            .bury(record.tx, record.entry, chunk_rate, reason);
        Some(TxRemoved { txid, reason, chunk_rate })
    }

    fn publish_additions(
        state: &mut State,
        transitions: &mut AddrTransitions,
        events: &mut Vec<TxAdded>,
        added: Vec<TxAddition>,
    ) {
        events.reserve(added.len());
        for addition in added {
            let kind = addition.kind();
            if let Some((tx, entry)) = Self::resolve_addition(state, addition) {
                events.push(TxAdded {
                    txid: entry.txid,
                    fee: entry.fee,
                    vsize: entry.vsize,
                    fee_rate: entry.fee_rate(),
                    first_seen: entry.first_seen,
                    kind,
                });
                Self::publish_one(state, transitions, tx, entry);
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

    fn publish_one(
        state: &mut State,
        transitions: &mut AddrTransitions,
        tx: Transaction,
        entry: TxEntry,
    ) {
        let prefix = entry.txid_prefix();
        state.info.add(&tx, entry.fee);
        state.addrs.add_tx(transitions, &tx);
        state.outpoint_spends.insert_spends(&tx, prefix);
        state.txs.insert(tx, entry);
    }
}
