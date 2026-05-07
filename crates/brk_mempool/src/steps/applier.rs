use brk_types::{Transaction, TxidPrefix};
use parking_lot::RwLock;

use crate::{
    TxEntry, TxRemoval,
    inner::MempoolInner,
    steps::preparer::{TxAddition, TxsPulled},
};

/// Applies a prepared diff to in-memory mempool state under one write
/// guard. Body proceeds: bury removed → publish added → evict.
pub struct Applier;

impl Applier {
    /// Returns true iff anything changed.
    pub fn apply(lock: &RwLock<MempoolInner>, pulled: TxsPulled) -> bool {
        let TxsPulled { added, removed } = pulled;
        let has_changes = !added.is_empty() || !removed.is_empty();

        let mut inner = lock.write();
        Self::bury_removals(&mut inner, removed);
        Self::publish_additions(&mut inner, added);
        inner.graveyard.evict_old();

        has_changes
    }

    fn bury_removals(inner: &mut MempoolInner, removed: Vec<(TxidPrefix, TxRemoval)>) {
        for (prefix, reason) in removed {
            Self::bury_one(inner, &prefix, reason);
        }
    }

    fn bury_one(inner: &mut MempoolInner, prefix: &TxidPrefix, reason: TxRemoval) {
        let Some(record) = inner.txs.remove_by_prefix(prefix) else {
            return;
        };
        let txid = record.entry.txid;
        inner.info.remove(&record.tx, record.entry.fee);
        inner.addrs.remove_tx(&record.tx, &txid);
        inner.outpoint_spends.remove_spends(&record.tx, *prefix);
        inner.graveyard.bury(txid, record.tx, record.entry, reason);
    }

    fn publish_additions(inner: &mut MempoolInner, added: Vec<TxAddition>) {
        for addition in added {
            if let Some((tx, entry)) = Self::resolve_addition(inner, addition) {
                Self::publish_one(inner, tx, entry);
            }
        }
    }

    fn resolve_addition(
        inner: &mut MempoolInner,
        addition: TxAddition,
    ) -> Option<(Transaction, TxEntry)> {
        match addition {
            TxAddition::Fresh { tx, entry } => Some((tx, entry)),
            TxAddition::Revived { entry } => {
                let tomb = inner.graveyard.exhume(&entry.txid)?;
                Some((tomb.tx, entry))
            }
        }
    }

    fn publish_one(inner: &mut MempoolInner, tx: Transaction, entry: TxEntry) {
        let prefix = entry.txid_prefix();
        inner.info.add(&tx, entry.fee);
        inner.addrs.add_tx(&tx, &entry.txid);
        inner.outpoint_spends.insert_spends(&tx, prefix);
        inner.txs.insert(tx, entry);
    }
}
