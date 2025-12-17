//! State recovery logic for checkpoint/resume.
//!
//! Determines starting height and imports saved state from checkpoints.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use brk_error::Result;
use brk_types::Height;
use vecdb::Stamp;

use super::super::address::AnyAddressIndexesVecs;
use super::super::cohorts::{AddressCohorts, UTXOCohorts};
use super::super::AddressesDataVecs;

/// Result of state recovery.
pub struct RecoveredState {
    /// Height to start processing from. Zero means fresh start.
    pub starting_height: Height,
}

/// Perform state recovery for resuming from checkpoint.
///
/// Rolls back state vectors and imports cohort states.
/// Validates that all rollbacks and imports are consistent.
/// Returns Height::ZERO if any validation fails (triggers fresh start).
pub fn recover_state(
    height: Height,
    chain_state_rollback: vecdb::Result<Stamp>,
    txoutindex_rollback: vecdb::Result<Stamp>,
    any_address_indexes: &mut AnyAddressIndexesVecs,
    addresses_data: &mut AddressesDataVecs,
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
) -> Result<RecoveredState> {
    let stamp = Stamp::from(height);

    // Rollback address state vectors
    let address_indexes_rollback = any_address_indexes.rollback_before(stamp);
    let address_data_rollback = addresses_data.rollback_before(stamp);

    // Verify rollback consistency - all must agree on the same height
    let consistent_height = rollback_states(
        chain_state_rollback,
        txoutindex_rollback,
        address_indexes_rollback,
        address_data_rollback,
    );

    // If rollbacks are inconsistent, start fresh
    if consistent_height.is_zero() {
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    // Import UTXO cohort states - all must succeed
    if !utxo_cohorts.import_separate_states(height) {
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    // Import address cohort states - all must succeed
    if !address_cohorts.import_separate_states(height) {
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    // Import aggregate price_to_amount - must match height
    let imported = import_aggregate_price_to_amount(height, utxo_cohorts)?;
    if imported != height {
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    Ok(RecoveredState {
        starting_height: height,
    })
}

/// Reset all state for fresh start.
///
/// Resets all state vectors and cohort states.
pub fn reset_state(
    any_address_indexes: &mut AnyAddressIndexesVecs,
    addresses_data: &mut AddressesDataVecs,
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
) -> Result<RecoveredState> {
    // Reset address state
    any_address_indexes.reset()?;
    addresses_data.reset()?;

    // Reset cohort state heights
    utxo_cohorts.reset_separate_state_heights();
    address_cohorts.reset_separate_state_heights();

    // Reset price_to_amount for all cohorts
    utxo_cohorts.reset_separate_price_to_amount()?;
    address_cohorts.reset_separate_price_to_amount()?;

    // Reset aggregate cohorts' price_to_amount
    utxo_cohorts.reset_aggregate_price_to_amount()?;

    Ok(RecoveredState {
        starting_height: Height::ZERO,
    })
}

/// Check if we can resume from a checkpoint or need to start fresh.
pub fn determine_start_mode(computed_min: Height, chain_state_height: Height) -> StartMode {
    // No data to resume from
    if chain_state_height.is_zero() {
        return StartMode::Fresh;
    }

    match computed_min.cmp(&chain_state_height) {
        Ordering::Greater => unreachable!("min height > chain state height"),
        Ordering::Equal => StartMode::Resume(chain_state_height),
        Ordering::Less => StartMode::Fresh,
    }
}

/// Whether to resume from checkpoint or start fresh.
pub enum StartMode {
    /// Resume from the given height.
    Resume(Height),
    /// Start from height 0.
    Fresh,
}

/// Rollback state vectors to before a given stamp.
///
/// Returns the consistent starting height if ALL rollbacks succeed and agree,
/// otherwise returns Height::ZERO (need fresh start).
fn rollback_states(
    chain_state_rollback: vecdb::Result<Stamp>,
    txoutindex_rollback: vecdb::Result<Stamp>,
    address_indexes_rollbacks: Result<Vec<Stamp>>,
    address_data_rollbacks: Result<[Stamp; 2]>,
) -> Height {
    let mut heights: BTreeSet<Height> = BTreeSet::new();

    // All rollbacks must succeed - any error means fresh start
    let Ok(s) = chain_state_rollback else {
        return Height::ZERO;
    };
    heights.insert(Height::from(s).incremented());

    let Ok(s) = txoutindex_rollback else {
        return Height::ZERO;
    };
    heights.insert(Height::from(s).incremented());

    let Ok(stamps) = address_indexes_rollbacks else {
        return Height::ZERO;
    };
    for s in stamps {
        heights.insert(Height::from(s).incremented());
    }

    let Ok(stamps) = address_data_rollbacks else {
        return Height::ZERO;
    };
    for s in stamps {
        heights.insert(Height::from(s).incremented());
    }

    // All must agree on the same height
    if heights.len() == 1 {
        heights.pop_first().unwrap()
    } else {
        Height::ZERO
    }
}

/// Import aggregate price_to_amount for UTXO cohorts.
fn import_aggregate_price_to_amount(
    starting_height: Height,
    utxo_cohorts: &mut UTXOCohorts,
) -> Result<Height> {
    if starting_height.is_zero() {
        return Ok(Height::ZERO);
    }

    let imported = utxo_cohorts.import_aggregate_price_to_amount(starting_height)?;

    Ok(if imported == starting_height {
        starting_height
    } else {
        Height::ZERO
    })
}

