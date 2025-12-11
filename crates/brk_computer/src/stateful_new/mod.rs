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

use process::*;

pub use vecs::Vecs;

// Address re-exports
pub use address::{
    AddressTypeToTypeIndexMap, AddressTypeToVec, AddressesDataVecs, AnyAddressIndexesVecs,
    HeightToAddressTypeToVec,
};

// Cohort re-exports
pub use cohorts::{
    AddressCohortVecs, AddressCohorts, CohortState, CohortVecs, DynCohortVecs, Flushable,
    HeightFlushable, UTXOCohortVecs, UTXOCohorts,
};

// Compute re-exports
pub use compute::{
    BIP30_DUPLICATE_HEIGHT_1, BIP30_DUPLICATE_HEIGHT_2, BIP30_ORIGINAL_HEIGHT_1,
    BIP30_ORIGINAL_HEIGHT_2, ComputeContext, FLUSH_INTERVAL, IndexerReaders, VecsReaders,
};

// Metrics re-exports
pub use metrics::{
    ActivityMetrics, CohortMetrics, ImportConfig, PricePaidMetrics, RealizedMetrics,
    RelativeMetrics, SupplyMetrics, UnrealizedMetrics,
};
