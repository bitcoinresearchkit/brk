//! Classification of txs that left the mempool between two pull cycles.
//!
//! `Replaced` = at least one added tx this cycle spends one of its
//! inputs (BIP-125 replacement inferred from conflicting outpoints).
//! `Vanished` = any other reason we can't distinguish from the data
//! at hand (mined, expired, evicted, or replaced by a tx we didn't
//! fetch due to the per-cycle fetch cap).

use brk_types::{Txid, TxidPrefix, Vout};
use rustc_hash::{FxHashMap, FxHashSet};

use super::added::Addition;
use crate::stores::TxStore;

#[derive(Debug)]
pub enum Removal {
    Replaced { by: Txid },
    Vanished,
}

/// Diff the store against Core's listing. `live` is the set of txid
/// prefixes Core returned this cycle; anything in `known` whose prefix
/// isn't in `live` left the pool. Each loser is classified by cross-
/// referencing its inputs against the freshly added txs' inputs.
pub(super) fn classify(
    live: &FxHashSet<TxidPrefix>,
    added: &[Addition],
    known: &TxStore,
) -> FxHashMap<TxidPrefix, Removal> {
    // (parent txid, vout) -> Txid of the new tx that spends it.
    // Only `Fresh` additions carry tx input data; revived txs were
    // already in-pool and can't be "new spenders" of anything.
    let mut spent_by: FxHashMap<(Txid, Vout), Txid> = FxHashMap::default();
    for addition in added {
        if let Addition::Fresh { tx, .. } = addition {
            for txin in &tx.input {
                spent_by.insert((txin.txid.clone(), txin.vout), tx.txid.clone());
            }
        }
    }

    known
        .iter()
        .filter_map(|(txid, tx)| {
            let prefix = TxidPrefix::from(txid);
            if live.contains(&prefix) {
                return None;
            }
            let removal = tx
                .input
                .iter()
                .find_map(|i| spent_by.get(&(i.txid.clone(), i.vout)).cloned())
                .map(|by| Removal::Replaced { by })
                .unwrap_or(Removal::Vanished);
            Some((prefix, removal))
        })
        .collect()
}
