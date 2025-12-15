//! Query implementation modules.
//!
//! Each module extends `Query` with domain-specific methods using `impl Query` blocks.

mod address;
mod block;
mod mempool;
mod metrics;
mod metrics_legacy;
mod mining;
mod transaction;

pub use block::BLOCK_TXS_PAGE_SIZE;
