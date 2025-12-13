use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::audit::{AuditTx, Pool};
use super::selection::select_into_blocks;
use crate::mempool::{MempoolEntry, MempoolTxIndex, PoolIndex, SelectedTx};

/// Number of projected blocks to build
const NUM_PROJECTED_BLOCKS: usize = 8;

/// Estimated txs per block (for partial sort optimization)
const TXS_PER_BLOCK: usize = 4000;

/// Build projected blocks from mempool entries.
///
/// Returns SelectedTx (with effective fee rate) grouped by block, in mining priority order.
pub fn build_projected_blocks(entries: &[Option<MempoolEntry>]) -> Vec<Vec<SelectedTx>> {
    // Collect live entries
    let live: Vec<(MempoolTxIndex, &MempoolEntry)> = entries
        .iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.as_ref().map(|e| (MempoolTxIndex::from(i), e)))
        .collect();

    if live.is_empty() {
        return Vec::new();
    }

    // Build AuditTx pool with pre-computed ancestor values from Bitcoin Core
    let mut pool = Pool::new(build_audit_pool(&live));

    // Sort by ancestor score (partial sort for efficiency)
    let sorted = partial_sort_by_score(&pool);

    // Run selection algorithm
    select_into_blocks(&mut pool, sorted, NUM_PROJECTED_BLOCKS)
}

/// Build the AuditTx pool with parent/child relationships.
/// AuditTx.parents and .children store pool indices (for graph traversal).
/// AuditTx.entries_idx stores the original entries index (for final output).
/// Uses Bitcoin Core's pre-computed ancestor values (correct, no double-counting).
fn build_audit_pool(live: &[(MempoolTxIndex, &MempoolEntry)]) -> Vec<AuditTx> {
    // Create mapping from TxidPrefix to pool index
    let prefix_to_pool_idx: FxHashMap<TxidPrefix, PoolIndex> = live
        .iter()
        .enumerate()
        .map(|(pool_idx, (_, entry))| (entry.txid_prefix(), PoolIndex::from(pool_idx)))
        .collect();

    // Build pool with parent relationships
    // Use Bitcoin Core's pre-computed ancestor_fee and ancestor_vsize
    let mut pool: Vec<AuditTx> = live
        .iter()
        .enumerate()
        .map(|(pool_idx, (entries_idx, entry))| {
            let pool_idx = PoolIndex::from(pool_idx);
            let mut tx = AuditTx::new_with_ancestors(
                *entries_idx,
                pool_idx,
                entry.fee,
                entry.vsize,
                entry.ancestor_fee,
                entry.ancestor_vsize,
            );

            // Find in-mempool parents from depends list (provided by Bitcoin Core)
            for parent_prefix in &entry.depends {
                if let Some(&parent_pool_idx) = prefix_to_pool_idx.get(parent_prefix) {
                    tx.parents.push(parent_pool_idx);
                }
            }

            tx
        })
        .collect();

    // Build child relationships (reverse of parents)
    for pool_idx in 0..pool.len() {
        let parents = pool[pool_idx].parents.clone();
        for parent_pool_idx in parents {
            pool[parent_pool_idx.as_usize()].children.push(PoolIndex::from(pool_idx));
        }
    }

    pool
}

/// Partial sort: only fully sort the top N txs needed for blocks.
/// Returns pool indices sorted by ancestor score.
fn partial_sort_by_score(pool: &Pool) -> Vec<PoolIndex> {
    let mut indices: Vec<PoolIndex> = (0..pool.len()).map(PoolIndex::from).collect();
    let needed = NUM_PROJECTED_BLOCKS * TXS_PER_BLOCK;

    // Comparator: descending by score, then ascending by index (deterministic tiebreaker)
    let cmp = |a: &PoolIndex, b: &PoolIndex| -> std::cmp::Ordering {
        let tx_a = &pool[*a];
        let tx_b = &pool[*b];
        if tx_b.has_higher_score_than(tx_a) {
            std::cmp::Ordering::Greater
        } else if tx_a.has_higher_score_than(tx_b) {
            std::cmp::Ordering::Less
        } else {
            a.cmp(b)
        }
    };

    if indices.len() > needed {
        // Partition: move top `needed` to front (unordered), then sort just those
        indices.select_nth_unstable_by(needed, cmp);
        indices[..needed].sort_unstable_by(cmp);
        indices.truncate(needed);
    } else {
        indices.sort_unstable_by(cmp);
    }

    indices
}
