//! Common vector structs and logic shared between UTXO and Address cohorts.
//!
//! This module contains the `Vecs` struct which holds all the computed vectors
//! for a single cohort, along with methods for importing, flushing, and computing.
//!
//! ## Module Organization
//!
//! The implementation is split across multiple files for maintainability:
//! - `vecs.rs`: Struct definition with field documentation
//! - `import.rs`: Import, validation, and initialization methods
//! - `push.rs`: Per-block push and flush methods
//! - `compute.rs`: Post-processing computation methods

mod compute;
mod import;
mod push;
mod vecs;

pub use vecs::Vecs;
