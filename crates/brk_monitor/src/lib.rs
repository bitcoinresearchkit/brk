//! Bitcoin mempool monitor.
//!
//! Provides real-time mempool tracking with:
//! - Fee estimation via projected blocks
//! - Address mempool stats
//! - CPFP-aware block building

mod mempool;

pub use mempool::{BlockStats, Mempool, MempoolInner, ProjectedSnapshot};
