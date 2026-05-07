//! Build the per-tx adjacency for a snapshot from the live `TxStore`.
//!
//! One pass over the live records to assign compact `TxIndex`es and a
//! `prefix -> TxIndex` map, then per entry resolve `depends` against
//! it to produce parent edges. Children are mirrored from parents in
//! a second pass. Cross-pool parents (confirmed or evicted) are
//! dropped silently - the live pool reflects what miners actually see,
//! and any stale `depends` entry is self-healing.
//!
//! The prefix map is returned alongside the txs so the rebuilder can
//! reuse it for GBT mapping and the final `Snapshot::build` step
//! without reconstructing it.

use brk_types::TxidPrefix;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;

use crate::TxEntry;
use crate::stores::TxStore;

use super::{SnapTx, TxIndex};

pub type PrefixIndex = FxHashMap<TxidPrefix, TxIndex>;

pub fn build_txs(txs: &TxStore) -> (Vec<SnapTx>, PrefixIndex) {
    if txs.is_empty() {
        return (Vec::new(), PrefixIndex::default());
    }

    let (prefix_to_idx, ordered) = compact_index(txs);
    let mut snap_txs: Vec<SnapTx> = ordered.iter().map(|e| live_tx(e, &prefix_to_idx)).collect();

    mirror_children(&mut snap_txs);
    (snap_txs, prefix_to_idx)
}

fn compact_index(txs: &TxStore) -> (PrefixIndex, Vec<&TxEntry>) {
    let mut map: PrefixIndex = FxHashMap::with_capacity_and_hasher(txs.len(), FxBuildHasher);
    let mut ordered: Vec<&TxEntry> = Vec::with_capacity(txs.len());
    for (i, (prefix, record)) in txs.records().enumerate() {
        map.insert(*prefix, TxIndex::from(i));
        ordered.push(&record.entry);
    }
    (map, ordered)
}

fn live_tx(e: &TxEntry, prefix_to_idx: &PrefixIndex) -> SnapTx {
    let parents: SmallVec<[TxIndex; 2]> = e
        .depends
        .iter()
        .filter_map(|p| prefix_to_idx.get(p).copied())
        .collect();
    SnapTx {
        txid: e.txid,
        fee: e.fee,
        vsize: e.vsize,
        weight: e.weight,
        size: e.size,
        chunk_rate: e.chunk_rate,
        parents,
        children: SmallVec::new(),
    }
}

fn mirror_children(txs: &mut [SnapTx]) {
    let edges: Vec<(TxIndex, TxIndex)> = txs
        .iter()
        .enumerate()
        .flat_map(|(i, t)| {
            let child = TxIndex::from(i);
            t.parents.iter().map(move |&p| (p, child))
        })
        .collect();
    for (parent, child) in edges {
        if let Some(t) = txs.get_mut(parent.as_usize()) {
            t.children.push(child);
        }
    }
}
