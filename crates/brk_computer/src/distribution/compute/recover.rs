use std::{cmp::Ordering, collections::BTreeSet};

use brk_error::Result;
use brk_types::Height;
use tracing::{debug, warn};
use vecdb::Stamp;

use super::super::{
    AddressesDataVecs,
    address::AnyAddressIndexesVecs,
    cohorts::{AddressCohorts, UTXOCohorts},
};

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
        address_indexes_rollback,
        address_data_rollback,
    );

    // If rollbacks are inconsistent, start fresh
    if consistent_height.is_zero() {
        warn!("Rollback consistency check failed: inconsistent heights");
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    // Rollback can land at an earlier height (multi-block change file), which is fine.
    // But if it lands AHEAD of target, that means rollback failed (missing change files).
    if consistent_height > height {
        warn!(
            "Rollback failed: still at {} but target was {}, falling back to fresh start",
            consistent_height, height
        );
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    if consistent_height != height {
        debug!(
            "Rollback landed at {} instead of {}, will resume from there",
            consistent_height, height
        );
    }

    // Import UTXO cohort states - all must succeed
    if !utxo_cohorts.import_separate_states(consistent_height) {
        warn!("UTXO cohort state import failed at height {}", consistent_height);
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    // Import address cohort states - all must succeed
    if !address_cohorts.import_separate_states(consistent_height) {
        warn!("Address cohort state import failed at height {}", consistent_height);
        return Ok(RecoveredState {
            starting_height: Height::ZERO,
        });
    }

    Ok(RecoveredState {
        starting_height: consistent_height,
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

    Ok(RecoveredState {
        starting_height: Height::ZERO,
    })
}

/// Check if we can resume from a checkpoint or need to start fresh.
///
/// - `min_available`: minimum height we have data for across all stateful vecs
/// - `resume_target`: the height we want to resume processing from
pub fn determine_start_mode(min_available: Height, resume_target: Height) -> StartMode {
    // No data to resume from
    if resume_target.is_zero() {
        return StartMode::Fresh;
    }

    match min_available.cmp(&resume_target) {
        Ordering::Greater => unreachable!("min_available > resume_target"),
        Ordering::Equal => StartMode::Resume(resume_target),
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
    address_indexes_rollbacks: Result<Vec<Stamp>>,
    address_data_rollbacks: Result<[Stamp; 2]>,
) -> Height {
    let mut heights: BTreeSet<Height> = BTreeSet::new();

    // All rollbacks must succeed - any error means fresh start
    let Ok(s) = chain_state_rollback else {
        warn!("chain_state rollback failed: {:?}", chain_state_rollback);
        return Height::ZERO;
    };
    let chain_height = Height::from(s).incremented();
    debug!("chain_state rolled back to stamp {:?}, height {}", s, chain_height);
    heights.insert(chain_height);

    let Ok(stamps) = address_indexes_rollbacks else {
        warn!("address_indexes rollback failed: {:?}", address_indexes_rollbacks);
        return Height::ZERO;
    };
    for (i, s) in stamps.iter().enumerate() {
        let h = Height::from(*s).incremented();
        debug!("address_indexes[{}] rolled back to stamp {:?}, height {}", i, s, h);
        heights.insert(h);
    }

    let Ok(stamps) = address_data_rollbacks else {
        warn!("address_data rollback failed: {:?}", address_data_rollbacks);
        return Height::ZERO;
    };
    for (i, s) in stamps.iter().enumerate() {
        let h = Height::from(*s).incremented();
        debug!("address_data[{}] rolled back to stamp {:?}, height {}", i, s, h);
        heights.insert(h);
    }

    // All must agree on the same height
    if heights.len() == 1 {
        heights.pop_first().unwrap()
    } else {
        warn!("Rollback heights inconsistent: {:?}", heights);
        Height::ZERO
    }
}
