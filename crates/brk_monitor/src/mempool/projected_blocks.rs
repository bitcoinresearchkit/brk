use brk_types::{FeeRate, RecommendedFees, Sats, Txid, VSize};
use rustc_hash::FxHashSet;

use super::TxGraph;

/// Maximum block weight in weight units (4 million)
const MAX_BLOCK_WEIGHT: u64 = 4_000_000;

/// Target block vsize (weight / 4)
const BLOCK_VSIZE_TARGET: u64 = MAX_BLOCK_WEIGHT / 4;

/// Number of projected blocks to build
const NUM_PROJECTED_BLOCKS: usize = 8;

/// A projected future block built from mempool transactions
#[derive(Debug, Clone, Default)]
pub struct ProjectedBlock {
    pub txids: Vec<Txid>,
    pub total_vsize: VSize,
    pub total_fee: Sats,
    pub min_fee_rate: FeeRate,
    pub max_fee_rate: FeeRate,
    pub median_fee_rate: FeeRate,
}

/// Projected mempool blocks for fee estimation
#[derive(Debug, Clone, Default)]
pub struct ProjectedBlocks {
    pub blocks: Vec<ProjectedBlock>,
}

impl ProjectedBlocks {
    /// Build projected blocks from a transaction graph
    ///
    /// Simulates how miners would construct blocks by selecting
    /// transactions with highest ancestor fee rates first.
    pub fn build(graph: &TxGraph) -> Self {
        if graph.is_empty() {
            return Self::default();
        }

        // Collect entries sorted by ancestor fee rate (descending)
        let mut sorted: Vec<_> = graph
            .entries()
            .iter()
            .map(|(txid, entry)| (txid.clone(), entry.ancestor_fee_rate(), entry.vsize, entry.fee))
            .collect();

        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        // Build blocks greedily
        let mut blocks = Vec::with_capacity(NUM_PROJECTED_BLOCKS);
        let mut current_block = ProjectedBlock::default();
        let mut included: FxHashSet<Txid> = FxHashSet::default();

        for (txid, fee_rate, vsize, fee) in sorted {
            // Skip if already included (as part of ancestor package)
            if included.contains(&txid) {
                continue;
            }

            // Would this tx fit in the current block?
            let new_vsize = current_block.total_vsize + vsize;

            if u64::from(new_vsize) > BLOCK_VSIZE_TARGET {
                // Finalize current block if it has transactions
                if !current_block.txids.is_empty() {
                    Self::finalize_block(&mut current_block);
                    blocks.push(current_block);

                    if blocks.len() >= NUM_PROJECTED_BLOCKS {
                        break;
                    }
                }

                // Start new block
                current_block = ProjectedBlock::default();
            }

            // Add to current block
            current_block.txids.push(txid.clone());
            current_block.total_vsize += vsize;
            current_block.total_fee += fee;
            included.insert(txid);

            // Track fee rate bounds
            if current_block.max_fee_rate == FeeRate::default() {
                current_block.max_fee_rate = fee_rate;
            }
            current_block.min_fee_rate = fee_rate;
        }

        // Don't forget the last block
        if !current_block.txids.is_empty() && blocks.len() < NUM_PROJECTED_BLOCKS {
            Self::finalize_block(&mut current_block);
            blocks.push(current_block);
        }

        Self { blocks }
    }

    /// Compute recommended fees from projected blocks
    pub fn recommended_fees(&self) -> RecommendedFees {
        RecommendedFees {
            fastest_fee: self.fee_for_block(0),
            half_hour_fee: self.fee_for_block(2),  // ~3 blocks
            hour_fee: self.fee_for_block(5),       // ~6 blocks
            economy_fee: self.fee_for_block(7),    // ~12 blocks, but we only have 8
            minimum_fee: 1.0,
        }
    }

    /// Get the minimum fee rate needed to get into block N
    fn fee_for_block(&self, block_index: usize) -> f64 {
        self.blocks
            .get(block_index)
            .map(|b| f64::from(b.min_fee_rate))
            .unwrap_or(1.0)
            .max(1.0) // Never recommend below 1 sat/vB
    }

    fn finalize_block(block: &mut ProjectedBlock) {
        // Compute median fee rate from min/max as approximation
        // (true median would require storing all fee rates)
        let min = f64::from(block.min_fee_rate);
        let max = f64::from(block.max_fee_rate);
        block.median_fee_rate = FeeRate::from((min + max) / 2.0);
    }
}
