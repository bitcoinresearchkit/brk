//! Block processing pipeline.
//!
//! This module handles the main computation loop that processes blocks:
//! 1. Recover state from checkpoint or start fresh
//! 2. Process each block's outputs and inputs
//! 3. Update cohort states
//! 4. Periodically flush to disk
//! 5. Compute aggregate cohorts from separate cohorts

mod aggregates;
mod block_loop;
mod context;
mod flush;
mod readers;
mod recover;

pub use aggregates::{compute_overlapping, compute_rest_part1, compute_rest_part2};
pub use block_loop::process_blocks;
pub use context::ComputeContext;
pub use flush::{flush_checkpoint, flush_cohort_states};
pub use readers::{
    IndexerReaders, VecsReaders, build_txinindex_to_txindex, build_txoutindex_to_txindex,
};
pub use recover::{
    RecoveredState, StartMode, determine_start_mode, find_min_height,
    import_aggregate_price_to_amount, import_cohort_states, reset_all_state, rollback_states,
};

/// Flush checkpoint interval (every N blocks).
pub const FLUSH_INTERVAL: usize = 10_000;

// BIP30 duplicate coinbase heights (special case handling)
pub const BIP30_DUPLICATE_HEIGHT_1: u32 = 91_842;
pub const BIP30_DUPLICATE_HEIGHT_2: u32 = 91_880;
pub const BIP30_ORIGINAL_HEIGHT_1: u32 = 91_812;
pub const BIP30_ORIGINAL_HEIGHT_2: u32 = 91_722;
