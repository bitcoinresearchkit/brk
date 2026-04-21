use std::cmp::Reverse;

use super::{BLOCK_VSIZE, package::Package};

/// How many packages to look ahead when the current one doesn't fit.
const LOOK_AHEAD_COUNT: usize = 100;

/// Partition packages into blocks by placement rate.
///
/// The first `num_blocks - 1` blocks are packed greedily into ~`BLOCK_VSIZE`
/// chunks. The final block is a catch-all containing every remaining
/// package, so no low-rate tx is silently dropped from the projection
/// (matches mempool.space's last-block behavior).
pub fn partition_into_blocks(
    mut packages: Vec<Package>,
    num_blocks: usize,
) -> Vec<Vec<Package>> {
    // Stable sort for deterministic output across equal placement rates.
    // Topology across dependent packages is already enforced by the
    // placement_rate cap in the selector.
    packages.sort_by_key(|p| Reverse(p.placement_rate));

    let mut slots: Vec<Option<Package>> = packages.into_iter().map(Some).collect();
    let mut blocks: Vec<Vec<Package>> = Vec::with_capacity(num_blocks);
    let normal_blocks = num_blocks.saturating_sub(1);

    let mut idx = fill_normal_blocks(&mut slots, &mut blocks, normal_blocks);

    if blocks.len() < num_blocks {
        let mut overflow: Vec<Package> = Vec::new();
        while idx < slots.len() {
            if let Some(pkg) = slots[idx].take() {
                overflow.push(pkg);
            }
            idx += 1;
        }
        if !overflow.is_empty() {
            blocks.push(overflow);
        }
    }

    blocks
}

/// Greedily pack packages into up to `target_blocks` chunks of `BLOCK_VSIZE`.
/// Returns the first `slots` index we stopped at.
fn fill_normal_blocks(
    slots: &mut [Option<Package>],
    blocks: &mut Vec<Vec<Package>>,
    target_blocks: usize,
) -> usize {
    let mut current_block: Vec<Package> = Vec::new();
    let mut current_vsize: u64 = 0;
    let mut idx = 0;

    while idx < slots.len() && blocks.len() < target_blocks {
        let Some(pkg) = &slots[idx] else {
            idx += 1;
            continue;
        };

        let remaining_space = BLOCK_VSIZE.saturating_sub(current_vsize);

        if pkg.vsize <= remaining_space {
            let pkg = slots[idx].take().unwrap();
            current_vsize += pkg.vsize;
            current_block.push(pkg);
            idx += 1;
            continue;
        }

        if current_block.is_empty() {
            // Oversized package with no partial block to preserve; take it
            // anyway so we don't stall on a package larger than BLOCK_VSIZE.
            let pkg = slots[idx].take().unwrap();
            current_vsize += pkg.vsize;
            current_block.push(pkg);
            idx += 1;
            continue;
        }

        if try_fill_with_smaller(
            slots,
            idx,
            remaining_space,
            &mut current_block,
            &mut current_vsize,
        ) {
            continue;
        }

        blocks.push(std::mem::take(&mut current_block));
        current_vsize = 0;
    }

    if !current_block.is_empty() && blocks.len() < target_blocks {
        blocks.push(current_block);
    }

    idx
}

/// Scan the look-ahead window for a package small enough to fit in the
/// remaining space and move it into the current block.
fn try_fill_with_smaller(
    slots: &mut [Option<Package>],
    start: usize,
    remaining_space: u64,
    block: &mut Vec<Package>,
    block_vsize: &mut u64,
) -> bool {
    let end = (start + LOOK_AHEAD_COUNT).min(slots.len());
    for idx in (start + 1)..end {
        let Some(pkg) = &slots[idx] else { continue };
        if pkg.vsize <= remaining_space {
            let pkg = slots[idx].take().unwrap();
            *block_vsize += pkg.vsize;
            block.push(pkg);
            return true;
        }
    }
    false
}
