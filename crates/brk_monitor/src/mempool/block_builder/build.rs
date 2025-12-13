use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::audit::{AuditTx, Pool};
use super::selection::select_into_blocks;
use crate::mempool::{MempoolEntry, MempoolTxIndex, PoolIndex, SelectedTx};

/// Number of projected blocks to build
const NUM_PROJECTED_BLOCKS: usize = 8;

/// Build projected blocks from mempool entries.
pub fn build_projected_blocks(entries: &[Option<MempoolEntry>]) -> Vec<Vec<SelectedTx>> {
    let live: Vec<(MempoolTxIndex, &MempoolEntry)> = entries
        .iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.as_ref().map(|e| (MempoolTxIndex::from(i), e)))
        .collect();

    if live.is_empty() {
        return Vec::new();
    }

    let mut pool = Pool::new(build_audit_pool(&live));
    select_into_blocks(&mut pool, NUM_PROJECTED_BLOCKS)
}

/// Build the AuditTx pool with parent/child relationships.
fn build_audit_pool(live: &[(MempoolTxIndex, &MempoolEntry)]) -> Vec<AuditTx> {
    // Map TxidPrefix -> pool index
    let prefix_to_idx: FxHashMap<TxidPrefix, PoolIndex> = live
        .iter()
        .enumerate()
        .map(|(i, (_, entry))| (entry.txid_prefix(), PoolIndex::from(i)))
        .collect();

    // Build pool with parent relationships
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

            // Add in-mempool parents
            for parent_prefix in &entry.depends {
                if let Some(&parent_idx) = prefix_to_idx.get(parent_prefix) {
                    tx.parents.push(parent_idx);
                }
            }

            tx
        })
        .collect();

    // Build child relationships (reverse of parents)
    for i in 0..pool.len() {
        let parents = pool[i].parents.clone();
        for parent_idx in parents {
            pool[parent_idx.as_usize()].children.push(PoolIndex::from(i));
        }
    }

    pool
}
