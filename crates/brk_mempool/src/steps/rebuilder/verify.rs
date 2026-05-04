use brk_rpc::Client;
use brk_types::{Sats, SatsSigned, TxidPrefix, VSize};
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::{debug, warn};

use crate::TxEntry;
use crate::cluster::{Cluster, ClusterRef};
use crate::stores::TxIndex;

type PrefixSet = FxHashSet<TxidPrefix>;
type FeeByPrefix = FxHashMap<TxidPrefix, Sats>;

pub struct Verifier;

impl Verifier {
    pub fn check(
        client: &Client,
        blocks: &[Vec<TxIndex>],
        clusters: &[Cluster<TxIndex>],
        cluster_of: &[Option<ClusterRef>],
        entries: &[Option<TxEntry>],
    ) {
        Self::check_structure(blocks, clusters, cluster_of, entries);
        Self::compare_to_core(client, blocks, entries);
    }

    fn check_structure(
        blocks: &[Vec<TxIndex>],
        clusters: &[Cluster<TxIndex>],
        cluster_of: &[Option<ClusterRef>],
        entries: &[Option<TxEntry>],
    ) {
        let in_pool: PrefixSet = entries
            .iter()
            .filter_map(|e| e.as_ref().map(TxEntry::txid_prefix))
            .collect();
        let mut placed = PrefixSet::default();

        for (b, block) in blocks.iter().enumerate() {
            let mut block_vsize = VSize::default();
            for &tx_index in block {
                let entry = Self::live_entry(entries, tx_index, b);
                Self::assert_parents_placed_first(entry, &in_pool, &placed, b);
                Self::place(entry, &mut placed, b);
                Self::assert_in_a_chunk(clusters, cluster_of, tx_index, b);
                block_vsize += entry.vsize;
            }
            if b + 1 < blocks.len() {
                Self::assert_block_fits_budget(block_vsize, block.len(), b);
            }
        }
    }

    fn assert_in_a_chunk(
        clusters: &[Cluster<TxIndex>],
        cluster_of: &[Option<ClusterRef>],
        tx_index: TxIndex,
        b: usize,
    ) {
        let cref = cluster_of[tx_index.as_usize()]
            .unwrap_or_else(|| panic!("block {b}: tx_index {tx_index:?} has no cluster"));
        let _ = clusters[cref.cluster_id.as_usize()].chunk_of(cref.local);
    }

    fn live_entry(entries: &[Option<TxEntry>], tx_index: TxIndex, b: usize) -> &TxEntry {
        entries[tx_index.as_usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("block {b}: dead tx_index {tx_index:?}"))
    }

    fn assert_parents_placed_first(
        entry: &TxEntry,
        in_pool: &PrefixSet,
        placed: &PrefixSet,
        b: usize,
    ) {
        for parent in &entry.depends {
            assert!(
                !in_pool.contains(parent) || placed.contains(parent),
                "block {b}: {} placed before its parent",
                entry.txid,
            );
        }
    }

    fn place(entry: &TxEntry, placed: &mut PrefixSet, b: usize) {
        assert!(
            placed.insert(entry.txid_prefix()),
            "block {b}: duplicate txid {}",
            entry.txid
        );
    }

    fn assert_block_fits_budget(total: VSize, tx_count: usize, b: usize) {
        let is_oversized_singleton = tx_count == 1 && total > VSize::MAX_BLOCK;
        if is_oversized_singleton {
            return;
        }
        assert!(
            total <= VSize::MAX_BLOCK,
            "block {b}: vsize {total} exceeds {}",
            VSize::MAX_BLOCK
        );
    }

    fn compare_to_core(client: &Client, blocks: &[Vec<TxIndex>], entries: &[Option<TxEntry>]) {
        let Some(next_block) = blocks.first() else {
            return;
        };
        let core: FeeByPrefix = match client.get_block_template_txs() {
            Ok(txs) => txs
                .into_iter()
                .map(|t| (TxidPrefix::from(&t.txid), t.fee))
                .collect(),
            Err(e) => {
                warn!("verify: getblocktemplate failed: {e}");
                return;
            }
        };
        let ours: FeeByPrefix = next_block
            .iter()
            .filter_map(|&i| entries[i.as_usize()].as_ref())
            .map(|e| (e.txid_prefix(), e.fee))
            .collect();

        let overlap = ours.keys().filter(|k| core.contains_key(k)).count();
        let union = ours.len() + core.len() - overlap;
        let jaccard = if union == 0 {
            1.0
        } else {
            overlap as f64 / union as f64
        };

        let ours_fee: Sats = ours.values().copied().sum();
        let core_fee: Sats = core.values().copied().sum();
        let delta = SatsSigned::from(ours_fee) - SatsSigned::from(core_fee);
        let delta_bps = if core_fee == Sats::ZERO {
            0.0
        } else {
            f64::from(delta) / f64::from(core_fee) * 10_000.0
        };

        debug!(
            "verify block 0: txs {}/{} (overlap {}, jaccard {:.3}) | fee {}/{} (delta {:+}, {:+.1} bps)",
            ours.len(),
            core.len(),
            overlap,
            jaccard,
            ours_fee,
            core_fee,
            delta.inner(),
            delta_bps,
        );
    }
}
