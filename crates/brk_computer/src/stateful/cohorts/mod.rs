//! Cohort management for UTXO and address groupings.
//!
//! Cohorts are groups of UTXOs or addresses filtered by criteria like:
//! - Age (0-1d, 1-7d, etc.)
//! - Amount (< 1 BTC, 1-10 BTC, etc.)
//! - Type (P2PKH, P2SH, etc.)
//! - Term (short-term holder, long-term holder)

mod address;
mod address_cohorts;
mod traits;
mod utxo;
mod utxo_cohorts;

pub use address::AddressCohortVecs;
pub use address_cohorts::AddressCohorts;
pub use traits::{CohortVecs, DynCohortVecs};
pub use utxo::UTXOCohortVecs;
pub use utxo_cohorts::UTXOCohorts;
