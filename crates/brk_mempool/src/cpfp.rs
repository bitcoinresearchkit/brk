//! CPFP (Child Pays For Parent) cluster reasoning for live mempool
//! transactions. Cluster scope is the seed's projected block: txs in
//! other projected blocks share no mining fate with the seed, so
//! including them in `effectiveFeePerVsize` would be misleading.
//!
//! Confirmed-tx CPFP (the same-block connected component on the
//! chain) lives in `brk_query`, since it reads indexer/computer vecs.

use brk_types::{CpfpEntry, CpfpInfo, FeeRate, Sats, TxidPrefix, VSize, Weight};
use rustc_hash::FxHashSet;

use crate::{Mempool, TxEntry};

/// Cap matches Bitcoin Core's default mempool ancestor/descendant
/// chain limits and `confirmed_cpfp`'s cap.
const MAX: usize = 25;

impl Mempool {
    /// CPFP info for a live mempool tx, scoped to the seed's projected
    /// block. Returns `None` if the tx is not in the mempool, so
    /// callers can fall through to the confirmed path. Returns `Some`
    /// with empty arms if the tx is in the mempool but below the
    /// projection floor (no projected block to share fate with).
    pub fn cpfp_info(&self, prefix: &TxidPrefix) -> Option<CpfpInfo> {
        let snapshot = self.snapshot();
        let entries = self.entries();
        let seed_idx = entries.idx_of(prefix)?;
        let seed = entries.slot(seed_idx)?;

        let mut sum_fee = u64::from(seed.fee);
        let mut sum_vsize = u64::from(seed.vsize);
        let mut ancestors: Vec<CpfpEntry> = Vec::new();
        let mut descendants: Vec<CpfpEntry> = Vec::new();

        if let Some(seed_block) = snapshot.block_of(seed_idx) {
            // Ancestor BFS gated to the seed's projected block.
            // `visited` dedupes the walk; stale parent prefixes
            // (confirmed/evicted between snapshot and now) are skipped
            // when `idx_of` returns None.
            let mut visited: FxHashSet<TxidPrefix> = FxHashSet::default();
            visited.insert(*prefix);
            let mut stack: Vec<TxidPrefix> = seed.depends.iter().copied().collect();
            while let Some(p) = stack.pop() {
                if ancestors.len() >= MAX {
                    break;
                }
                if !visited.insert(p) {
                    continue;
                }
                let Some(idx) = entries.idx_of(&p) else { continue };
                if snapshot.block_of(idx) != Some(seed_block) {
                    continue;
                }
                let Some(anc) = entries.slot(idx) else { continue };
                sum_fee += u64::from(anc.fee);
                sum_vsize += u64::from(anc.vsize);
                ancestors.push(to_entry(anc));
                stack.extend(anc.depends.iter().copied());
            }

            // Descendant sweep. `desc_set` starts with only the seed
            // so siblings (txs sharing an ancestor with seed but not
            // downstream of it) are excluded. The topological ordering
            // of `Snapshot.blocks` guarantees that all in-block
            // ancestors of any tx are visited before it.
            let mut desc_set: FxHashSet<TxidPrefix> = FxHashSet::default();
            desc_set.insert(*prefix);
            for &i in &snapshot.blocks[seed_block.as_usize()] {
                if descendants.len() >= MAX {
                    break;
                }
                let Some(e) = entries.slot(i) else { continue };
                if !e.depends.iter().any(|d| desc_set.contains(d)) {
                    continue;
                }
                desc_set.insert(e.txid_prefix());
                sum_fee += u64::from(e.fee);
                sum_vsize += u64::from(e.vsize);
                descendants.push(to_entry(e));
            }
        }

        let best_descendant = descendants
            .iter()
            .max_by_key(|e| FeeRate::from((e.fee, e.weight)))
            .cloned();

        let package_rate = FeeRate::from((Sats::from(sum_fee), VSize::from(sum_vsize)));
        let effective = seed.fee_rate().max(package_rate);

        Some(CpfpInfo {
            ancestors,
            best_descendant,
            descendants,
            effective_fee_per_vsize: Some(effective),
            fee: Some(seed.fee),
            adjusted_vsize: Some(seed.vsize),
        })
    }
}

fn to_entry(e: &TxEntry) -> CpfpEntry {
    CpfpEntry {
        txid: e.txid.clone(),
        weight: Weight::from(e.vsize),
        fee: e.fee,
    }
}
