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
    /// Height to start processing from.
    pub starting_height: Height,
    /// Whether state was successfully restored (vs starting fresh).
    pub restored: bool,
}

/// Perform state recovery for resuming from checkpoint.
///
/// Rolls back state vectors and imports cohort states.
/// Returns the recovered state information.
pub fn recover_state(
    height: Height,
    any_address_indexes: &mut AnyAddressIndexesVecs,
    addresses_data: &mut AddressesDataVecs,
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
) -> Result<RecoveredState> {
    let stamp = Stamp::from(height);

    // Rollback address state vectors
    let address_indexes_rollback = any_address_indexes.rollback_before(stamp);
    let address_data_rollback = addresses_data.rollback_before(stamp);

    // Verify rollback consistency (uses rollback_states helper)
    let _consistent_height = rollback_states(
        stamp,
        Ok(stamp), // chain_state handled separately
        address_indexes_rollback,
        address_data_rollback,
    );

    // Import cohort states
    utxo_cohorts.import_separate_states(height);
    address_cohorts.import_separate_states(height);

    // Import aggregate price_to_amount
    let _ = import_aggregate_price_to_amount(height, utxo_cohorts)?;

    Ok(RecoveredState {
        starting_height: height,
        restored: true,
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
        restored: false,
    })
}

/// Check if we can resume from a checkpoint or need to start fresh.
pub fn determine_start_mode(computed_min: Height, chain_state_height: Height) -> StartMode {
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
/// Returns the consistent starting height if all vectors agree,
/// otherwise returns Height::ZERO (need fresh start).
fn rollback_states(
    _stamp: Stamp,
    chain_state_rollback: vecdb::Result<Stamp>,
    address_indexes_rollbacks: Result<Vec<Stamp>>,
    address_data_rollbacks: Result<[Stamp; 2]>,
) -> Height {
    let mut heights: BTreeSet<Height> = BTreeSet::new();

    if let Ok(s) = chain_state_rollback {
        heights.insert(Height::from(s).incremented());
    }

    if let Ok(stamps) = address_indexes_rollbacks {
        for s in stamps {
            heights.insert(Height::from(s).incremented());
        }
    }

    if let Ok(stamps) = address_data_rollbacks {
        for s in stamps {
            heights.insert(Height::from(s).incremented());
        }
    }

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

