//! Why a tx left the mempool between two pull cycles, plus the
//! classifier that diffs the live prefix set against `known` to
//! produce one [`TxRemoval`] per loser.

use brk_types::{Transaction, Txid, TxidPrefix, Vout};
use rustc_hash::{FxHashMap, FxHashSet};

use super::TxAddition;
use crate::stores::TxStore;

/// `Replaced` = at least one freshly added tx this cycle spends one of
/// its inputs (BIP-125 replacement inferred from conflicting outpoints).
/// `Vanished` = any other reason we can't distinguish from the data at
/// hand (mined, expired, evicted, or replaced by a tx we didn't fetch
/// due to the per-cycle fetch cap).
#[derive(Debug)]
pub enum TxRemoval {
    Replaced { by: Txid },
    Vanished,
}

type SpentBy = FxHashMap<(Txid, Vout), Txid>;

impl TxRemoval {
    /// Returns `(prefix, reason)` pairs in iteration order of `known`.
    pub(super) fn classify(
        live: &FxHashSet<TxidPrefix>,
        added: &[TxAddition],
        known: &TxStore,
    ) -> Vec<(TxidPrefix, Self)> {
        let spent_by = Self::build_spent_by(added);

        known
            .iter()
            .filter_map(|(txid, tx)| {
                let prefix = TxidPrefix::from(txid);
                if live.contains(&prefix) {
                    return None;
                }
                Some((prefix, Self::find_removal(tx, &spent_by)))
            })
            .collect()
    }

    /// `Replaced` if any of `tx`'s inputs is now claimed by a freshly
    /// added tx (BIP-125 inferred); otherwise `Vanished`.
    fn find_removal(tx: &Transaction, spent_by: &SpentBy) -> Self {
        tx.input
            .iter()
            .find_map(|i| spent_by.get(&(i.txid.clone(), i.vout)).cloned())
            .map_or(Self::Vanished, |by| Self::Replaced { by })
    }

    /// Only `Fresh` additions carry tx input data. Revived txs were
    /// already in-pool, so they can't be new spenders of anything.
    fn build_spent_by(added: &[TxAddition]) -> SpentBy {
        let mut spent_by: SpentBy = FxHashMap::default();
        for addition in added {
            if let TxAddition::Fresh { tx, .. } = addition {
                for txin in &tx.input {
                    spent_by.insert((txin.txid.clone(), txin.vout), tx.txid.clone());
                }
            }
        }
        spent_by
    }
}
