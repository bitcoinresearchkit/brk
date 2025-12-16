//! Stateful computation for Bitcoin UTXO and address cohort metrics.
//!
//! This module processes blockchain data to compute metrics for various cohorts
//! (groups of UTXOs or addresses filtered by age, amount, type, etc.).
//!
//! ## Module Structure
//!
//! ```text
//! stateful/
//! ├── address/        # Address type handling (indexes, data storage)
//! ├── cohorts/        # Cohort traits and state management
//! ├── compute/        # Block processing pipeline
//! └── metrics/        # Metric vectors organized by category
//! ```
//!
//! ## Data Flow
//!
//! 1. **Import**: Load from checkpoint or start fresh
//! 2. **Process blocks**: For each block, process outputs/inputs in parallel
//! 3. **Update cohorts**: Track supply, realized/unrealized P&L per cohort
//! 4. **Flush**: Periodically checkpoint state to disk
//! 5. **Compute aggregates**: Derive aggregate cohorts from separate cohorts

pub mod address;
pub mod cohorts;
pub mod compute;
pub mod metrics;
mod process;
mod vecs;


pub use vecs::Vecs;

// Address re-exports
pub use address::{
    AddressTypeToTypeIndexMap, AddressesDataVecs, AnyAddressIndexesVecs,
};

// Cohort re-exports
pub use cohorts::{
    AddressCohorts, CohortVecs, DynCohortVecs, Flushable, UTXOCohorts,
};

// Compute re-exports
pub use compute::IndexerReaders;

// Metrics re-exports
