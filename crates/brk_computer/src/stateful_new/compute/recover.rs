//! State recovery logic for checkpoint/resume.
//!
//! Determines starting height and imports saved state from checkpoints.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use brk_error::Result;
use brk_types::Height;
use vecdb::{AnyVec, Stamp};

use super::super::address::AnyAddressIndexesVecs;
use super::super::cohorts::{DynCohortVecs, UTXOCohorts};

/// Result of state recovery.
pub struct RecoveredState {
    /// Height to start processing from.
    pub starting_height: Height,
    /// Whether state was successfully restored (vs starting fresh).
    pub restored: bool,
}

/// Determine starting height from vector lengths.
pub fn find_min_height(
    utxo_vecs: &[&mut dyn DynCohortVecs],
    address_vecs: &[&mut dyn DynCohortVecs],
    chain_state_len: usize,
    address_indexes_min_height: Height,
    address_data_min_height: Height,
    other_vec_lens: &[usize],
) -> Height {
    let utxo_min = utxo_vecs
        .iter()
        .map(|v| Height::from(v.min_height_vecs_len()))
        .min()
        .unwrap_or_default();

    let address_min = address_vecs
        .iter()
        .map(|v| Height::from(v.min_height_vecs_len()))
        .min()
        .unwrap_or_default();

    let other_min = other_vec_lens
        .iter()
        .map(|&len| Height::from(len))
        .min()
        .unwrap_or_default();

    utxo_min
        .min(address_min)
        .min(Height::from(chain_state_len))
        .min(address_indexes_min_height)
        .min(address_data_min_height)
        .min(other_min)
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
pub fn rollback_states(
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

/// Import state for all separate cohorts.
///
/// Returns the starting height if all imports succeed with the same height,
/// otherwise returns Height::ZERO.
pub fn import_cohort_states(
    starting_height: Height,
    cohorts: &mut [&mut dyn DynCohortVecs],
) -> Height {
    if starting_height.is_zero() {
        return Height::ZERO;
    }

    let all_match = cohorts
        .iter_mut()
        .map(|v| v.import_state(starting_height).unwrap_or_default())
        .all(|h| h == starting_height);

    if all_match {
        starting_height
    } else {
        Height::ZERO
    }
}

/// Import aggregate price_to_amount for UTXO cohorts.
pub fn import_aggregate_price_to_amount(
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

/// Reset all state for fresh start.
pub fn reset_all_state(
    address_indexes: &mut AnyAddressIndexesVecs,
    utxo_vecs: &mut [&mut dyn DynCohortVecs],
    address_vecs: &mut [&mut dyn DynCohortVecs],
    utxo_cohorts: &mut UTXOCohorts,
) -> Result<()> {
    address_indexes.reset()?;

    for v in utxo_vecs.iter_mut() {
        v.reset_state_starting_height();
    }

    for v in address_vecs.iter_mut() {
        v.reset_state_starting_height();
    }

    utxo_cohorts.reset_aggregate_price_to_amount()?;

    Ok(())
}
