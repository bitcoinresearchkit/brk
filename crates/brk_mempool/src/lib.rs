//! Bitcoin mempool monitor with fee estimation.
//!
//! Provides real-time mempool tracking with:
//! - Fee estimation via projected blocks
//! - Address mempool stats
//! - CPFP-aware block building

mod addresses;
mod block_builder;
mod entry;
mod entry_pool;
mod projected_blocks;
mod sync;
mod tx_store;
mod types;

pub use projected_blocks::{BlockStats, RecommendedFees, Snapshot};
pub use sync::{Mempool, MempoolInner};
