use super::package::Package;
use super::BLOCK_VSIZE;
use crate::types::SelectedTx;

/// How many packages to look ahead when current doesn't fit.
const LOOK_AHEAD: usize = 100;

/// Partition packages into blocks by fee rate.
///
/// Packages are sorted by fee rate descending, then placed into blocks.
/// When a package doesn't fit, we look ahead for smaller packages that do.
/// Atomic packages are never split across blocks.
pub fn partition_into_blocks(mut packages: Vec<Package>, num_blocks: usize) -> Vec<Vec<SelectedTx>> {
    packages.sort_unstable_by(|a, b| b.fee_rate.cmp(&a.fee_rate));

    let mut blocks: Vec<Vec<SelectedTx>> = Vec::with_capacity(num_blocks);
    let mut current_block: Vec<SelectedTx> = Vec::new();
    let mut current_vsize: u64 = 0;
    let mut used = vec![false; packages.len()];

    let mut idx = 0;
    while idx < packages.len() && blocks.len() < num_blocks {
        if used[idx] {
            idx += 1;
            continue;
        }

        let remaining_space = BLOCK_VSIZE.saturating_sub(current_vsize);
        let package = &packages[idx];

        if package.vsize <= remaining_space {
            current_block.extend(package.txs.iter().copied());
            current_vsize += package.vsize;
            used[idx] = true;
            idx += 1;
            continue;
        }

        // Package doesn't fit
        if current_block.is_empty() {
            // Empty block: add oversized package anyway
            current_block.extend(package.txs.iter().copied());
            current_vsize += package.vsize;
            used[idx] = true;
            idx += 1;
            continue;
        }

        // Look ahead for a smaller package that fits
        let found_smaller = try_fill_with_smaller(
            &packages,
            &mut used,
            idx,
            remaining_space,
            &mut current_block,
            &mut current_vsize,
        );

        if !found_smaller {
            // No package fits, finalize current block
            blocks.push(std::mem::take(&mut current_block));
            current_vsize = 0;
        }
    }

    if !current_block.is_empty() && blocks.len() < num_blocks {
        blocks.push(current_block);
    }

    blocks
}

/// Try to find a smaller package in the look-ahead window that fits.
fn try_fill_with_smaller(
    packages: &[Package],
    used: &mut [bool],
    start: usize,
    remaining_space: u64,
    block: &mut Vec<SelectedTx>,
    block_vsize: &mut u64,
) -> bool {
    let end = (start + LOOK_AHEAD).min(packages.len());

    for idx in (start + 1)..end {
        if used[idx] {
            continue;
        }

        let package = &packages[idx];
        if package.vsize <= remaining_space {
            block.extend(package.txs.iter().copied());
            *block_vsize += package.vsize;
            used[idx] = true;
            return true;
        }
    }

    false
}
