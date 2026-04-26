use brk_types::{MempoolInfo, Transaction, Txid};

use crate::{
    steps::preparer::{Addition, Pulled},
    stores::{AddrTracker, EntryPool, TxGraveyard, TxStore},
};

/// Applies a prepared diff to in-memory mempool state.
///
/// Removals are torn down first: each tx+entry is moved into the
/// graveyard with its removal reason.
///
/// Additions then publish to live state. For `Revived` additions the
/// tx body is exhumed from the graveyard (no clone); for `Fresh` ones
/// the tx arrives inline from the Preparer.
///
/// Finally the graveyard evicts entries past its retention window.
pub struct Applier;

impl Applier {
    /// Apply `pulled` to all buckets. Returns true if anything changed.
    pub fn apply(
        pulled: Pulled,
        info: &mut MempoolInfo,
        txs: &mut TxStore,
        addrs: &mut AddrTracker,
        entries: &mut EntryPool,
        graveyard: &mut TxGraveyard,
    ) -> bool {
        let Pulled { added, removed } = pulled;
        let has_changes = !added.is_empty() || !removed.is_empty();

        for (prefix, reason) in removed {
            let Some(entry) = entries.remove(&prefix) else {
                continue;
            };
            let txid = entry.txid.clone();
            let Some(tx) = txs.remove(&txid) else {
                continue;
            };
            info.remove(&tx, entry.fee);
            addrs.remove_tx(&tx, &txid);
            graveyard.bury(txid, tx, entry, reason);
        }

        let mut to_store: Vec<(Txid, Transaction)> = Vec::with_capacity(added.len());
        for addition in added {
            let (tx, entry) = match addition {
                Addition::Fresh { tx, entry } => (tx, entry),
                Addition::Revived { entry } => {
                    let Some(tomb) = graveyard.exhume(&entry.txid) else {
                        continue;
                    };
                    (tomb.tx, entry)
                }
            };
            info.add(&tx, entry.fee);
            addrs.add_tx(&tx, &entry.txid);
            let txid = entry.txid.clone();
            entries.insert(entry);
            to_store.push((txid, tx));
        }
        txs.extend(to_store);

        graveyard.evict_old();

        has_changes
    }
}
