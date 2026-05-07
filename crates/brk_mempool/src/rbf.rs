//! RBF tree extraction. Returns owned trees so the caller can enrich
//! with indexer data (`mined`, effective fee rate) after the lock
//! drops: enriching under the lock re-enters `Mempool` and would
//! recursively acquire the same read locks.

use brk_types::{Sats, Timestamp, Transaction, Txid, TxidPrefix, VSize};
use rustc_hash::FxHashSet;

use crate::{
    Mempool, TxEntry, TxRemoval, TxStore,
    stores::{EntryPool, TxGraveyard},
};

#[derive(Debug, Clone)]
pub struct RbfNode {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
    pub value: Sats,
    pub first_seen: Timestamp,
    /// BIP-125 signaling: at least one input has sequence < 0xffffffff-1.
    pub rbf: bool,
    /// `true` iff any predecessor in this subtree was non-signaling.
    pub full_rbf: bool,
    pub replaces: Vec<RbfNode>,
}

#[derive(Debug, Clone, Default)]
pub struct RbfForTx {
    /// Tree rooted at the terminal replacer. `None` if `txid` is unknown.
    pub root: Option<RbfNode>,
    /// Direct predecessors of the requested tx (txids only).
    pub replaces: Vec<Txid>,
}

impl Mempool {
    /// Walk forward through `Replaced { by }` to the terminal replacer
    /// and return its full predecessor tree, plus the requested tx's
    /// direct predecessors. Single read-lock window in canonical order.
    pub fn rbf_for_tx(&self, txid: &Txid) -> RbfForTx {
        let txs = self.txs();
        let entries = self.entries();
        let graveyard = self.graveyard();

        let root_txid = walk_to_replacement_root(&graveyard, *txid);
        let replaces: Vec<Txid> = graveyard.predecessors_of(txid).map(|(p, _)| *p).collect();
        let root = build_node(&root_txid, &txs, &entries, &graveyard);
        RbfForTx { root, replaces }
    }

    /// Recent terminal-replacer trees, most-recent first, deduplicated
    /// by root, capped at `limit`. `full_rbf_only` drops trees with no
    /// non-signaling predecessor.
    pub fn recent_rbf_trees(&self, full_rbf_only: bool, limit: usize) -> Vec<RbfNode> {
        let txs = self.txs();
        let entries = self.entries();
        let graveyard = self.graveyard();

        let mut seen: FxHashSet<Txid> = FxHashSet::default();
        graveyard
            .replaced_iter_recent_first()
            .filter_map(|(_, by)| {
                let root = walk_to_replacement_root(&graveyard, *by);
                seen.insert(root).then_some(root)
            })
            .filter_map(|root| build_node(&root, &txs, &entries, &graveyard))
            .filter(|n| !full_rbf_only || n.full_rbf)
            .take(limit)
            .collect()
    }
}

fn walk_to_replacement_root(graveyard: &TxGraveyard, mut root: Txid) -> Txid {
    while let Some(TxRemoval::Replaced { by }) = graveyard.get(&root).map(|t| t.reason()) {
        root = *by;
    }
    root
}

fn build_node(
    txid: &Txid,
    txs: &TxStore,
    entries: &EntryPool,
    graveyard: &TxGraveyard,
) -> Option<RbfNode> {
    let (tx, entry) = resolve_node(txid, txs, entries, graveyard)?;

    let replaces: Vec<RbfNode> = graveyard
        .predecessors_of(txid)
        .filter_map(|(pred, _)| build_node(pred, txs, entries, graveyard))
        .collect();

    let full_rbf = replaces.iter().any(|c| !c.rbf || c.full_rbf);
    let value: Sats = tx.output.iter().map(|o| o.value).sum();

    Some(RbfNode {
        txid: *txid,
        fee: entry.fee,
        vsize: entry.vsize,
        value,
        first_seen: entry.first_seen,
        rbf: entry.rbf,
        full_rbf,
        replaces,
    })
}

fn resolve_node<'a>(
    txid: &Txid,
    txs: &'a TxStore,
    entries: &'a EntryPool,
    graveyard: &'a TxGraveyard,
) -> Option<(&'a Transaction, &'a TxEntry)> {
    if let (Some(tx), Some(entry)) = (txs.get(txid), entries.get(&TxidPrefix::from(txid))) {
        return Some((tx, entry));
    }
    graveyard.get(txid).map(|tomb| (&tomb.tx, &tomb.entry))
}
