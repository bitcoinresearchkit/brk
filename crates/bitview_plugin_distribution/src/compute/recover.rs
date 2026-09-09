use brk_error::Result;
use brk_types::Height;
use tracing::{debug, warn};
use vecdb::{Result as VecdbResult, Stamp};

use super::super::{
    Vecs,
    state::{AddrStates, UTXOStates},
};

impl Vecs {
    /// Perform state recovery for resuming from checkpoint.
    ///
    /// Rolls back state vectors and imports cohort states.
    /// Validates that all rollbacks and imports are consistent.
    /// Returns Height::ZERO if any validation fails (triggers fresh start).
    pub fn recover_state(
        &mut self,
        height: Height,
        chain_state_rollback: Option<VecdbResult<Stamp>>,
        utxo_states: &mut UTXOStates,
        addr_states: &mut AddrStates,
    ) -> Result<Height> {
        // `None`: clean resume, already at the checkpoint, nothing to undo.
        // `Some`: reorg, undo state past the resume point.
        let consistent_height = match chain_state_rollback {
            None => height,
            Some(chain_state_rollback) => {
                let stamp = Stamp::from(height);

                // Rollback address state vectors
                let addr_state_rollback = self.addr_state.rollback_before(stamp);

                // Verify rollback consistency - all must agree on the same height
                let consistent_height = rollback_states(chain_state_rollback, addr_state_rollback);

                // If rollbacks are inconsistent, start fresh
                if consistent_height.is_zero() {
                    return Ok(Height::ZERO);
                }

                // Rollback can land at an earlier height (multi-block change file), which is fine.
                // But if it lands AHEAD of target, that means rollback failed (missing change files).
                if consistent_height > height {
                    warn!(
                        "Distribution rollback stopped at block {} instead of {}; rebuilding from genesis",
                        consistent_height, height
                    );
                    return Ok(Height::ZERO);
                }

                if consistent_height != height {
                    debug!(
                        "Rollback landed at {} instead of {}, will resume from there",
                        consistent_height, height
                    );
                }

                consistent_height
            }
        };

        // Import UTXO cohort states - all must succeed
        debug!(
            "importing UTXO cohort states at height {}",
            consistent_height
        );
        if !utxo_states.import(&self.cohorts, consistent_height)? {
            warn!(
                "Could not restore UTXO distribution state at block {}; rebuilding from genesis",
                consistent_height
            );
            return Ok(Height::ZERO);
        }
        debug!("UTXO cohort states imported");

        // Import address cohort states - all must succeed
        debug!(
            "importing addr cohort states at height {}",
            consistent_height
        );
        if !addr_states.import(&self.cohorts, &self.addrs.funded, consistent_height)? {
            warn!(
                "Could not restore address distribution state at block {}; rebuilding from genesis",
                consistent_height
            );
            return Ok(Height::ZERO);
        }
        debug!("addr cohort states imported");

        Ok(consistent_height)
    }
}

/// Check if we can resume from a checkpoint or need to start fresh.
///
/// - `min_resume_len`: minimum length across the vectors required by the block loop
/// - `resume_target`: the height we want to resume processing from
pub fn determine_start_mode(min_resume_len: Height, resume_target: Height) -> StartMode {
    if resume_target.is_zero() || min_resume_len < resume_target {
        StartMode::Fresh
    } else {
        StartMode::Resume(resume_target)
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
    chain_state_rollback: VecdbResult<Stamp>,
    addr_state_rollbacks: Result<Vec<Stamp>>,
) -> Height {
    // All rollbacks must succeed - any error means fresh start
    let s = match chain_state_rollback {
        Ok(stamp) => stamp,
        Err(error) => {
            warn!("Could not roll back distribution chain state; rebuilding from genesis: {error}");
            return Height::ZERO;
        }
    };
    let chain_height = Height::from(s).incremented();
    debug!(
        "chain_state rolled back to stamp {:?}, height {}",
        s, chain_height
    );

    let stamps = match addr_state_rollbacks {
        Ok(stamps) => stamps,
        Err(error) => {
            warn!(
                "Could not roll back distribution address state; rebuilding from genesis: {error}"
            );
            return Height::ZERO;
        }
    };
    let mut consistent = true;
    for (i, s) in stamps.iter().enumerate() {
        let h = Height::from(*s).incremented();
        debug!(
            "addr_state[{}] rolled back to stamp {:?}, height {}",
            i, s, h
        );
        consistent &= h == chain_height;
    }

    // All must agree on the same height
    if consistent {
        chain_height
    } else {
        warn!("Distribution rollback checkpoints disagree; rebuilding from genesis");
        Height::ZERO
    }
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use brk_types::Height;
    use vecdb::Stamp;

    use super::{StartMode, determine_start_mode, rollback_states};

    #[test]
    fn rollback_requires_every_checkpoint_to_agree() {
        let stamp = Stamp::from(Height::new(99));
        for count in [0, 1, 10] {
            assert_eq!(
                rollback_states(Ok(stamp), Ok(vec![stamp; count])),
                Height::new(100)
            );
        }
        for mismatch in [98, 100] {
            assert_eq!(
                rollback_states(
                    Ok(stamp),
                    Ok(vec![stamp, Stamp::from(Height::new(mismatch)), stamp])
                ),
                Height::ZERO
            );
        }
        assert_eq!(
            rollback_states(
                Err(Error::other("chain rollback failed").into()),
                Ok(vec![stamp])
            ),
            Height::ZERO
        );
        assert_eq!(
            rollback_states(
                Ok(stamp),
                Err(Error::other("address rollback failed").into())
            ),
            Height::ZERO
        );
    }

    #[test]
    fn resumes_when_required_vectors_reach_checkpoint() {
        let target = Height::new(100);
        assert!(matches!(
            determine_start_mode(target, target),
            StartMode::Resume(height) if height == target
        ));
    }

    #[test]
    fn resumes_at_earlier_reorg_target() {
        let target = Height::new(90);
        assert!(matches!(
            determine_start_mode(Height::new(100), target),
            StartMode::Resume(height) if height == target
        ));
    }

    #[test]
    fn starts_fresh_when_a_resume_vector_is_short() {
        assert!(matches!(
            determine_start_mode(Height::new(99), Height::new(100)),
            StartMode::Fresh
        ));
    }

    #[test]
    fn starts_fresh_without_checkpoint() {
        assert!(matches!(
            determine_start_mode(Height::ZERO, Height::ZERO),
            StartMode::Fresh
        ));
    }
}
