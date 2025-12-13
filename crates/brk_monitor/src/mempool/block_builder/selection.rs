use std::collections::BinaryHeap;

use brk_types::FeeRate;
use rustc_hash::FxHashSet;

use super::audit::{Pool, TxPriority};
use crate::mempool::{MempoolTxIndex, PoolIndex, SelectedTx};

/// Target vsize per block (~1MB, derived from 4MW weight limit)
const BLOCK_VSIZE_LIMIT: u64 = 1_000_000;

/// How many packages to look ahead when current doesn't fit
const LOOK_AHEAD: usize = 100;

/// A CPFP package - atomic unit that must be included together.
struct Package {
    /// Transactions in topological order (parents before children)
    txs: Vec<(MempoolTxIndex, FeeRate)>,
    /// Combined vsize of all txs
    vsize: u64,
    /// Package fee rate (same for all txs)
    fee_rate: FeeRate,
}

/// Select transactions into projected blocks.
///
/// Algorithm:
/// 1. Select txs via heap, grouping CPFP chains into atomic packages
/// 2. Sort packages by fee rate
/// 3. Partition into blocks (packages are atomic, never split)
pub fn select_into_blocks(pool: &mut Pool, num_blocks: usize) -> Vec<Vec<SelectedTx>> {
    // Phase 1: Select and group into packages
    let packages = select_packages(pool, num_blocks);

    // Phase 2: Partition packages into blocks
    partition_into_blocks(packages, num_blocks)
}

/// Select txs and group CPFP chains into atomic packages.
fn select_packages(pool: &mut Pool, num_blocks: usize) -> Vec<Package> {
    let target_vsize = BLOCK_VSIZE_LIMIT * num_blocks as u64;
    let mut total_vsize: u64 = 0;
    let mut packages: Vec<Package> = Vec::new();

    let mut heap: BinaryHeap<TxPriority> = (0..pool.len())
        .map(|i| TxPriority::new(&pool[PoolIndex::from(i)]))
        .collect();

    while let Some(entry) = heap.pop() {
        let tx = &pool[entry.pool_idx];

        if tx.used || entry.is_stale(tx) {
            continue;
        }

        // Package rate at selection time
        let package_rate = FeeRate::from((tx.ancestor_fee, tx.ancestor_vsize));

        // Select this tx and all unselected ancestors (parents first)
        let ancestors = select_with_ancestors(pool, entry.pool_idx);

        let mut package_vsize: u64 = 0;
        let mut txs = Vec::with_capacity(ancestors.len());

        for sel_idx in ancestors {
            let vsize = u64::from(pool[sel_idx].vsize);
            txs.push((pool[sel_idx].entries_idx, package_rate));
            package_vsize += vsize;

            update_descendants(pool, sel_idx, &mut heap);
        }

        total_vsize += package_vsize;
        packages.push(Package {
            txs,
            vsize: package_vsize,
            fee_rate: package_rate,
        });

        if total_vsize >= target_vsize {
            break;
        }
    }

    packages
}

/// Sort packages by fee rate and partition into blocks.
fn partition_into_blocks(mut packages: Vec<Package>, num_blocks: usize) -> Vec<Vec<SelectedTx>> {
    // Sort by fee rate descending
    packages.sort_unstable_by(|a, b| b.fee_rate.cmp(&a.fee_rate));

    // Debug: show top and bottom packages after sorting
    log::info!("=== Top 10 packages after sorting ===");
    for (i, pkg) in packages.iter().take(10).enumerate() {
        log::info!(
            "  #{}: rate={:.4} sat/vB, vsize={}, txs={}",
            i,
            f64::from(pkg.fee_rate),
            pkg.vsize,
            pkg.txs.len()
        );
    }
    log::info!("=== Bottom 10 packages after sorting ===");
    let start = packages.len().saturating_sub(10);
    for (i, pkg) in packages.iter().skip(start).enumerate() {
        log::info!(
            "  #{}: rate={:.4} sat/vB, vsize={}, txs={}",
            start + i,
            f64::from(pkg.fee_rate),
            pkg.vsize,
            pkg.txs.len()
        );
    }

    let mut blocks: Vec<Vec<SelectedTx>> = Vec::with_capacity(num_blocks);
    let mut current_block: Vec<SelectedTx> = Vec::new();
    let mut current_block_packages: Vec<(FeeRate, u64, usize)> = Vec::new(); // for debug
    let mut current_vsize: u64 = 0;
    let mut used = vec![false; packages.len()];

    let mut i = 0;
    while i < packages.len() && blocks.len() < num_blocks {
        if used[i] {
            i += 1;
            continue;
        }

        let remaining = BLOCK_VSIZE_LIMIT.saturating_sub(current_vsize);

        if packages[i].vsize <= remaining {
            // Package fits in current block
            current_block_packages.push((packages[i].fee_rate, packages[i].vsize, packages[i].txs.len()));
            add_package_to_block(&packages[i], &mut current_block);
            current_vsize += packages[i].vsize;
            used[i] = true;
            i += 1;
        } else if current_block.is_empty() {
            // Empty block - add package anyway (handles edge case)
            current_block_packages.push((packages[i].fee_rate, packages[i].vsize, packages[i].txs.len()));
            add_package_to_block(&packages[i], &mut current_block);
            current_vsize += packages[i].vsize;
            used[i] = true;
            i += 1;
        } else {
            // Look ahead for a smaller package that fits
            let mut found = false;
            let look_ahead_end = (i + LOOK_AHEAD).min(packages.len());

            for j in (i + 1)..look_ahead_end {
                if used[j] || packages[j].vsize > remaining {
                    continue;
                }
                log::debug!(
                    "Look-ahead: adding pkg #{} (rate={:.4}) to block {} (min so far={:.4})",
                    j,
                    f64::from(packages[j].fee_rate),
                    blocks.len() + 1,
                    current_block_packages.iter().map(|(r, _, _)| f64::from(*r)).fold(f64::INFINITY, f64::min)
                );
                current_block_packages.push((packages[j].fee_rate, packages[j].vsize, packages[j].txs.len()));
                add_package_to_block(&packages[j], &mut current_block);
                current_vsize += packages[j].vsize;
                used[j] = true;
                found = true;
                break;
            }

            if !found {
                // No package fits, start new block
                log_block_debug(blocks.len() + 1, &current_block_packages);
                blocks.push(std::mem::take(&mut current_block));
                current_block_packages.clear();
                current_vsize = 0;
            }
        }
    }

    if !current_block.is_empty() && blocks.len() < num_blocks {
        log_block_debug(blocks.len() + 1, &current_block_packages);
        blocks.push(current_block);
    }

    blocks
}

fn log_block_debug(block_num: usize, packages: &[(FeeRate, u64, usize)]) {
    if packages.is_empty() {
        return;
    }
    let mut sorted: Vec<_> = packages.to_vec();
    sorted.sort_by(|a, b| b.0.cmp(&a.0));

    log::info!("=== Block {} - {} packages ===", block_num, packages.len());
    log::info!("  Top 5:");
    for (rate, vsize, txs) in sorted.iter().take(5) {
        log::info!("    rate={:.4} sat/vB, vsize={}, txs={}", f64::from(*rate), vsize, txs);
    }
    log::info!("  Bottom 5:");
    let start = sorted.len().saturating_sub(5);
    for (rate, vsize, txs) in sorted.iter().skip(start) {
        log::info!("    rate={:.4} sat/vB, vsize={}, txs={}", f64::from(*rate), vsize, txs);
    }
}

fn add_package_to_block(package: &Package, block: &mut Vec<SelectedTx>) {
    for (entries_idx, effective_fee_rate) in &package.txs {
        block.push(SelectedTx {
            entries_idx: *entries_idx,
            effective_fee_rate: *effective_fee_rate,
        });
    }
}

/// Select a tx and all its unselected ancestors in topological order.
fn select_with_ancestors(pool: &mut Pool, pool_idx: PoolIndex) -> Vec<PoolIndex> {
    let mut result: Vec<PoolIndex> = Vec::new();
    let mut stack: Vec<(PoolIndex, bool)> = vec![(pool_idx, false)];

    while let Some((idx, parents_done)) = stack.pop() {
        if pool[idx].used {
            continue;
        }

        if parents_done {
            pool[idx].used = true;
            result.push(idx);
        } else {
            stack.push((idx, true));
            for &parent in &pool[idx].parents {
                if !pool[parent].used {
                    stack.push((parent, false));
                }
            }
        }
    }

    result
}

/// Update descendants' ancestor scores after selecting a tx.
fn update_descendants(pool: &mut Pool, selected_idx: PoolIndex, heap: &mut BinaryHeap<TxPriority>) {
    let selected_fee = pool[selected_idx].fee;
    let selected_vsize = pool[selected_idx].vsize;

    // Track visited to avoid double-updates in diamond patterns
    let mut visited: FxHashSet<PoolIndex> = FxHashSet::default();
    let mut stack: Vec<PoolIndex> = pool[selected_idx].children.to_vec();

    while let Some(child_idx) = stack.pop() {
        if !visited.insert(child_idx) {
            continue;
        }

        let child = &mut pool[child_idx];
        if child.used {
            continue;
        }

        // Update ancestor totals
        child.ancestor_fee -= selected_fee;
        child.ancestor_vsize -= selected_vsize;

        // Increment generation and re-push to heap
        child.generation += 1;
        heap.push(TxPriority::new(child));

        // Continue to grandchildren
        stack.extend(child.children.iter().copied());
    }
}
