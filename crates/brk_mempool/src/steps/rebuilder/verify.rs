use brk_rpc::Client;
use brk_types::{Sats, SatsSigned, TxidPrefix, VSize};
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::{debug, warn};

use super::linearize::Package;
use crate::{TxEntry, stores::TxIndex};

type PrefixSet = FxHashSet<TxidPrefix>;
type FeeByPrefix = FxHashMap<TxidPrefix, Sats>;

pub struct Verifier;

impl Verifier {
    pub fn check(client: &Client, blocks: &[Vec<Package>], entries: &[Option<TxEntry>]) {
        Self::check_structure(blocks, entries);
        Self::compare_to_core(client, blocks, entries);
    }

    fn check_structure(blocks: &[Vec<Package>], entries: &[Option<TxEntry>]) {
        let in_pool: PrefixSet = entries
            .iter()
            .filter_map(|e| e.as_ref().map(TxEntry::txid_prefix))
            .collect();
        let mut placed = PrefixSet::default();

        for (b, block) in blocks.iter().enumerate() {
            for (p, pkg) in block.iter().enumerate() {
                let mut summed_vsize = VSize::default();
                for &tx_index in &pkg.txs {
                    let entry = Self::live_entry(entries, tx_index, b, p);
                    Self::assert_parents_placed_first(entry, &in_pool, &placed, b, p);
                    Self::place(entry, &mut placed, b, p);
                    summed_vsize += entry.vsize;
                }
                assert_eq!(
                    pkg.vsize, summed_vsize,
                    "block {b} pkg {p}: pkg.vsize {} != sum {summed_vsize}",
                    pkg.vsize
                );
            }
            if b + 1 < blocks.len() {
                Self::assert_block_fits_budget(block, b);
            }
        }
    }

    fn live_entry(entries: &[Option<TxEntry>], tx_index: TxIndex, b: usize, p: usize) -> &TxEntry {
        entries[tx_index.as_usize()]
            .as_ref()
            .unwrap_or_else(|| panic!("block {b} pkg {p}: dead tx_index {tx_index:?}"))
    }

    fn assert_parents_placed_first(
        entry: &TxEntry,
        in_pool: &PrefixSet,
        placed: &PrefixSet,
        b: usize,
        p: usize,
    ) {
        for parent in &entry.depends {
            assert!(
                !in_pool.contains(parent) || placed.contains(parent),
                "block {b} pkg {p}: {} placed before its parent",
                entry.txid,
            );
        }
    }

    fn place(entry: &TxEntry, placed: &mut PrefixSet, b: usize, p: usize) {
        assert!(
            placed.insert(entry.txid_prefix()),
            "block {b} pkg {p}: duplicate txid {}",
            entry.txid
        );
    }

    fn assert_block_fits_budget(block: &[Package], b: usize) {
        let total: VSize = block.iter().map(|pkg| pkg.vsize).sum();
        let is_oversized_singleton = block.len() == 1 && total > VSize::MAX_BLOCK;
        if is_oversized_singleton {
            return;
        }
        assert!(
            total <= VSize::MAX_BLOCK,
            "block {b}: vsize {total} exceeds {}",
            VSize::MAX_BLOCK
        );
    }

    fn compare_to_core(client: &Client, blocks: &[Vec<Package>], entries: &[Option<TxEntry>]) {
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
            .flat_map(|pkg| &pkg.txs)
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
