use std::collections::BinaryHeap;

use brk_types::FeeRate;

use super::audit::{Pool, TxPriority};
use crate::mempool::{PoolIndex, SelectedTx};

/// Target vsize per block (~1MB, derived from 4MW weight limit)
const BLOCK_VSIZE_LIMIT: u64 = 1_000_000;

/// Select transactions into blocks using the two-source algorithm.
///
/// Takes pool indices (sorted by score), returns SelectedTx with effective fee rate at selection time.
pub fn select_into_blocks(
    pool: &mut Pool,
    sorted_pool_indices: Vec<PoolIndex>,
    num_blocks: usize,
) -> Vec<Vec<SelectedTx>> {
    let mut blocks: Vec<Vec<SelectedTx>> = Vec::with_capacity(num_blocks);
    let mut current_block: Vec<SelectedTx> = Vec::new();
    let mut current_vsize: u64 = 0;

    let mut sorted_iter = sorted_pool_indices.into_iter().peekable();
    let mut modified_queue: BinaryHeap<TxPriority> = BinaryHeap::new();

    'outer: loop {
        // Pick best candidate from either sorted list or modified queue
        let best_pool_idx = pick_best_candidate(pool, &mut sorted_iter, &mut modified_queue);
        let Some(pool_idx) = best_pool_idx else {
            break;
        };

        // Skip if already used
        if pool[pool_idx].used {
            continue;
        }

        // Capture the package rate BEFORE selecting ancestors
        // This is the rate that justified this tx (and its ancestors) for inclusion
        let package_rate = {
            let tx = &pool[pool_idx];
            FeeRate::from((tx.ancestor_fee, tx.ancestor_vsize))
        };

        // Select this tx and all its unselected ancestors
        let selected = select_with_ancestors(pool, pool_idx);

        for sel_pool_idx in selected {
            let tx = &pool[sel_pool_idx];
            let tx_vsize = u64::from(tx.vsize);

            // Check if tx fits in current block
            if current_vsize + tx_vsize > BLOCK_VSIZE_LIMIT && !current_block.is_empty() {
                blocks.push(std::mem::take(&mut current_block));
                current_vsize = 0;

                if blocks.len() >= num_blocks {
                    // Early exit - we have enough blocks
                    break 'outer;
                }
            }

            // Effective fee rate = the package rate at selection time.
            // This is the mining score that determined which block this tx lands in.
            // For CPFP, both parent and child get the same package rate (the child's score).
            current_block.push(SelectedTx {
                entries_idx: tx.entries_idx,
                effective_fee_rate: package_rate,
            });
            current_vsize += tx_vsize;

            // Update descendants' ancestor scores
            update_descendants(pool, sel_pool_idx, &mut modified_queue);
        }
    }

    // Don't forget the last block
    if !current_block.is_empty() && blocks.len() < num_blocks {
        blocks.push(current_block);
    }

    // Post-process: fix fee rate ordering violations between adjacent blocks.
    // This handles cases where a tx's score improved after its target block was full.
    fix_block_ordering(&mut blocks);

    // Log how many txs were left unselected
    let total_selected: usize = blocks.iter().map(|b| b.len()).sum();
    log::debug!(
        "Selected {} txs into {} blocks, modified_queue has {} remaining",
        total_selected,
        blocks.len(),
        modified_queue.len()
    );

    blocks
}

/// Pick the best candidate from sorted list or modified queue.
/// Returns a pool index.
fn pick_best_candidate(
    pool: &Pool,
    sorted_iter: &mut std::iter::Peekable<std::vec::IntoIter<PoolIndex>>,
    modified_queue: &mut BinaryHeap<TxPriority>,
) -> Option<PoolIndex> {
    // Skip used txs in sorted iterator
    while sorted_iter.peek().is_some_and(|&idx| pool[idx].used) {
        sorted_iter.next();
    }

    // Skip used txs and stale entries in modified queue.
    // A tx can be pushed multiple times as its score improves (when different ancestors are selected).
    // For example: tx C depends on A and B. When A is selected, C is pushed with score 2.0.
    // When B is selected, C is pushed again with score 4.0. The queue now has two entries for C.
    // We skip the stale 2.0 entry and use the current 4.0 entry.
    while let Some(p) = modified_queue.peek() {
        let tx = &pool[p.pool_idx];
        if tx.used {
            modified_queue.pop();
            continue;
        }
        // Check if this queue entry has outdated snapshot (a newer entry exists with better score)
        if p.ancestor_fee != tx.ancestor_fee || p.ancestor_vsize != tx.ancestor_vsize {
            modified_queue.pop();
            continue;
        }
        break;
    }

    let sorted_best = sorted_iter.peek().map(|&idx| &pool[idx]);
    let modified_best = modified_queue.peek().map(|p| &pool[p.pool_idx]);

    match (sorted_best, modified_best) {
        (None, None) => None,
        (Some(_), None) => sorted_iter.next(),
        (None, Some(_)) => {
            let p = modified_queue.pop().unwrap();
            Some(p.pool_idx)
        }
        (Some(sorted_tx), Some(modified_tx)) => {
            // Compare CURRENT scores from pool (not stale snapshots)
            if sorted_tx.has_higher_score_than(modified_tx) {
                sorted_iter.next()
            } else {
                let p = modified_queue.pop().unwrap();
                Some(p.pool_idx)
            }
        }
    }
}

/// Select a tx and all its unselected ancestors (topological order).
/// Takes and returns pool indices.
fn select_with_ancestors(pool: &mut Pool, pool_idx: PoolIndex) -> Vec<PoolIndex> {
    let mut to_select: Vec<PoolIndex> = Vec::new();
    let mut stack = vec![pool_idx];

    // DFS to find all unselected ancestors
    while let Some(current) = stack.pop() {
        let tx = &pool[current];
        if tx.used {
            continue;
        }

        // Push unselected parents onto stack (process parents first)
        let has_unselected_parents = tx.parents.iter().any(|&p| !pool[p].used);
        if has_unselected_parents {
            stack.push(current); // Re-add self to process after parents
            for &parent in &tx.parents {
                if !pool[parent].used {
                    stack.push(parent);
                }
            }
        } else {
            // All parents selected, can select this one
            if !pool[current].used {
                pool[current].used = true;
                to_select.push(current);
            }
        }
    }

    to_select
}

/// Fix fee rate ordering violations between blocks.
/// Swaps txs between adjacent blocks until Block N's min >= Block N+1's max.
fn fix_block_ordering(blocks: &mut [Vec<SelectedTx>]) {
    // Iterate until no more swaps needed
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;

        for i in 0..blocks.len().saturating_sub(1) {
            // Find min in block i and max in block i+1
            let Some(curr_min_idx) = blocks[i]
                .iter()
                .enumerate()
                .min_by_key(|(_, s)| s.effective_fee_rate)
                .map(|(idx, _)| idx)
            else {
                continue;
            };

            let Some(next_max_idx) = blocks[i + 1]
                .iter()
                .enumerate()
                .max_by_key(|(_, s)| s.effective_fee_rate)
                .map(|(idx, _)| idx)
            else {
                continue;
            };

            let curr_min = blocks[i][curr_min_idx].effective_fee_rate;
            let next_max = blocks[i + 1][next_max_idx].effective_fee_rate;

            // If violation exists, swap the two txs
            if next_max > curr_min {
                // Swap: move high-fee tx to earlier block, low-fee tx to later block
                let high_tx = blocks[i + 1].swap_remove(next_max_idx);
                let low_tx = blocks[i].swap_remove(curr_min_idx);
                blocks[i].push(high_tx);
                blocks[i + 1].push(low_tx);
                changed = true;
            }
        }
    }

    if iterations >= MAX_ITERATIONS {
        log::warn!("fix_block_ordering: reached max iterations, some violations may remain");
    }
}

/// Update descendants' ancestor scores after selecting a tx.
/// Takes a pool index.
fn update_descendants(
    pool: &mut Pool,
    selected_pool_idx: PoolIndex,
    modified_queue: &mut BinaryHeap<TxPriority>,
) {
    let selected_fee = pool[selected_pool_idx].fee;
    let selected_vsize = pool[selected_pool_idx].vsize;

    // Track visited to avoid double-subtracting in diamond patterns
    let mut visited = rustc_hash::FxHashSet::default();

    // BFS through children (children are pool indices)
    let mut stack: Vec<PoolIndex> = pool[selected_pool_idx].children.to_vec();

    while let Some(child_idx) = stack.pop() {
        // Skip if already visited (handles diamond patterns)
        if !visited.insert(child_idx) {
            continue;
        }

        let child = &mut pool[child_idx];

        if child.used {
            continue;
        }

        // Subtract selected tx from ancestor totals
        child.ancestor_fee -= selected_fee;
        child.ancestor_vsize -= selected_vsize;

        // Always re-push to modified queue with updated score.
        // This may create duplicates, but we handle that by checking
        // if the tx is used or if the snapshot is stale when popping.
        modified_queue.push(TxPriority::new(child));
        child.in_modified = true;

        // Continue to grandchildren
        stack.extend(child.children.iter().copied());
    }
}
