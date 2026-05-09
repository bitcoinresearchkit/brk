//! Build the per-tx adjacency for a snapshot from the live `TxStore`,
//! then linearize chunk rates over every multi-tx cluster.
//!
//! One pass over the live records to assign compact `TxIndex`es and a
//! `prefix -> TxIndex` map, then per entry resolve `depends` against
//! it to produce parent edges. Children are mirrored from parents in a
//! second pass. Cross-pool parents (confirmed or evicted) are dropped
//! silently - the live pool reflects what miners actually see, and any
//! stale `depends` entry is self-healing.
//!
//! Final pass: walk every connected component and run Single Fee
//! Linearization over it (see [`crate::cluster`]); each member's
//! `chunk_rate` is overwritten with its chunk's feerate. Singletons
//! keep the `fee/vsize` seed set in `live_tx`.
//!
//! The prefix map is returned alongside the txs so the rebuilder can
//! reuse it for GBT mapping and the final `Snapshot::build` step
//! without reconstructing it.

use std::mem;

use brk_types::TxidPrefix;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;

use crate::{
    TxEntry,
    cluster::{linearize_component, walk_cluster},
    stores::TxStore,
};

use super::{SnapTx, TxIndex};

pub type PrefixIndex = FxHashMap<TxidPrefix, TxIndex>;

pub fn build_txs(txs: &TxStore) -> (Vec<SnapTx>, PrefixIndex) {
    let n = txs.len();
    let mut prefix_to_idx: PrefixIndex =
        FxHashMap::with_capacity_and_hasher(n, FxBuildHasher);
    for (i, (prefix, _)) in txs.records().enumerate() {
        prefix_to_idx.insert(*prefix, TxIndex::from(i));
    }
    let mut snap_txs: Vec<SnapTx> = txs
        .records()
        .map(|(_, record)| live_tx(&record.entry, &prefix_to_idx))
        .collect();

    mirror_children(&mut snap_txs);
    refresh_chunk_rates(&mut snap_txs);
    (snap_txs, prefix_to_idx)
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
        chunk_rate: e.fee_rate(),
        parents,
        children: SmallVec::new(),
    }
}

fn mirror_children(txs: &mut [SnapTx]) {
    for i in 0..txs.len() {
        let child = TxIndex::from(i);
        let parents = mem::take(&mut txs[i].parents);
        for &p in &parents {
            if let Some(t) = txs.get_mut(p.as_usize()) {
                t.children.push(child);
            }
        }
        txs[i].parents = parents;
    }
}

/// Walk every multi-tx connected component once and overwrite each
/// member's `chunk_rate` with the linearized chunk's feerate. Visited
/// bitmap ensures each cluster is linearized exactly once.
fn refresh_chunk_rates(snap_txs: &mut [SnapTx]) {
    let n = snap_txs.len();
    let mut visited = vec![false; n];
    for seed in 0..n {
        if visited[seed] {
            continue;
        }
        let t = &snap_txs[seed];
        if t.parents.is_empty() && t.children.is_empty() {
            visited[seed] = true;
            continue;
        }
        let component = walk_cluster(snap_txs, TxIndex::from(seed));
        for &m in &component {
            visited[m.as_usize()] = true;
        }
        if component.len() <= 1 {
            continue;
        }
        let (members, chunks) = linearize_component(snap_txs, &component);
        for chunk in &chunks {
            for &local in &chunk.txs {
                let m = members[u32::from(local) as usize];
                snap_txs[m.as_usize()].chunk_rate = chunk.feerate;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use bitcoin::hashes::Hash;
    use brk_types::{FeeRate, Sats, Txid, VSize, Weight};

    use super::*;

    /// Build a `SnapTx` for tests. `txid` is auto-assigned from a
    /// process-wide counter so each tx is distinguishable in
    /// debug output; the cluster code itself keys off `TxIndex`,
    /// not `txid`.
    fn snap_tx(fee: Sats, vsize: VSize) -> SnapTx {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let mut bytes = [0u8; 32];
        bytes[..4].copy_from_slice(&COUNTER.fetch_add(1, Ordering::Relaxed).to_le_bytes());
        SnapTx {
            txid: Txid::from(bitcoin::Txid::from_byte_array(bytes)),
            fee,
            vsize,
            weight: Weight::from(vsize),
            size: u64::from(vsize),
            chunk_rate: FeeRate::from((fee, vsize)),
            parents: SmallVec::new(),
            children: SmallVec::new(),
        }
    }

    fn link(txs: &mut [SnapTx], parent: usize, child: usize) {
        txs[child].parents.push(TxIndex::from(parent));
        txs[parent].children.push(TxIndex::from(child));
    }

    fn sats(n: u64) -> Sats {
        Sats::from(n)
    }

    fn vsize(n: u64) -> VSize {
        VSize::from(n)
    }

    #[test]
    fn singleton_keeps_fee_per_vsize() {
        let mut txs = vec![snap_tx(sats(1000), vsize(100))];
        let seed = txs[0].chunk_rate;
        refresh_chunk_rates(&mut txs);
        assert_eq!(txs[0].chunk_rate, seed);
    }

    #[test]
    fn two_tx_cpfp_lift() {
        let mut txs = vec![
            snap_tx(sats(100), vsize(100)),
            snap_tx(sats(1900), vsize(100)),
        ];
        link(&mut txs, 0, 1);
        let parent_seed = txs[0].chunk_rate;
        refresh_chunk_rates(&mut txs);
        assert!(txs[0].chunk_rate > parent_seed);
        assert_eq!(txs[0].chunk_rate, txs[1].chunk_rate);
        assert_eq!(txs[0].chunk_rate, FeeRate::from((sats(2000), vsize(200))));
    }

    #[test]
    fn three_tx_chain_chunks_correctly() {
        let mut txs = vec![
            snap_tx(sats(100), vsize(100)),
            snap_tx(sats(100), vsize(100)),
            snap_tx(sats(5800), vsize(100)),
        ];
        link(&mut txs, 0, 1);
        link(&mut txs, 1, 2);
        refresh_chunk_rates(&mut txs);
        let combined = FeeRate::from((sats(6000), vsize(300)));
        assert_eq!(txs[0].chunk_rate, combined);
        assert_eq!(txs[1].chunk_rate, combined);
        assert_eq!(txs[2].chunk_rate, combined);
    }

    #[test]
    fn disjoint_clusters_linearized_independently() {
        let mut txs = vec![
            snap_tx(sats(100), vsize(100)),
            snap_tx(sats(1900), vsize(100)),
            snap_tx(sats(500), vsize(100)),
            snap_tx(sats(4500), vsize(100)),
        ];
        link(&mut txs, 0, 1);
        link(&mut txs, 2, 3);
        refresh_chunk_rates(&mut txs);
        assert_eq!(txs[0].chunk_rate, txs[1].chunk_rate);
        assert_eq!(txs[2].chunk_rate, txs[3].chunk_rate);
        assert_ne!(txs[0].chunk_rate, txs[2].chunk_rate);
    }

    #[test]
    fn cluster_cap_does_not_panic() {
        let n = 100;
        let mut txs: Vec<SnapTx> = (0..n).map(|_| snap_tx(sats(1000), vsize(100))).collect();
        for i in 1..n {
            link(&mut txs, i - 1, i);
        }
        refresh_chunk_rates(&mut txs);
    }
}
