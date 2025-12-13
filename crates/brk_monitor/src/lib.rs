//! Bitcoin mempool monitor.
//!
//! Provides real-time mempool tracking with:
//! - Fee estimation via projected blocks
//! - Address mempool stats
//! - CPFP-aware block building

mod block_builder;
mod mempool;
mod projected_blocks;
mod types;

pub use mempool::{Mempool, MempoolInner};
pub use projected_blocks::{BlockStats, RecommendedFees, Snapshot};
