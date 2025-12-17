//! Age-based state transitions for UTXO cohorts.
//!
//! When a new block arrives, UTXOs age. Some cross day boundaries
//! and need to move between age-based cohorts.

use brk_grouper::{Filter, Filtered, UTXOGroups};
use brk_types::{ONE_DAY_IN_SEC, Sats, Timestamp};

use crate::{PriceToAmount, states::BlockState, utils::OptionExt};

use super::UTXOCohorts;

impl UTXOCohorts {
    /// Handle age transitions when processing a new block.
    ///
    /// UTXOs age with each block. When they cross day boundaries,
    /// they move between age-based cohorts (e.g., from "0-1d" to "1-7d").
    pub fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;

        // Optimization: Only blocks whose age % ONE_DAY >= threshold can cross a day boundary.
        // Saves computation vs checking days_old for every block.
        let elapsed = (*timestamp).saturating_sub(*prev_timestamp);
        let threshold = ONE_DAY_IN_SEC.saturating_sub(elapsed);

        // Extract mutable references to avoid borrow checker issues
        let UTXOGroups {
            all,
            term,
            age_range,
            ..
        } = &mut self.0;

        // Collect age_range cohorts with their filters and states
        let mut age_cohorts: Vec<(Filter, &mut Option<_>)> = age_range
            .iter_mut()
            .map(|v| (v.filter().clone(), &mut v.state))
            .collect();

        // Collect aggregate cohorts' price_to_amount for age transitions
        let mut aggregate_p2a: Vec<(Filter, Option<&mut PriceToAmount>)> = vec![
            (all.filter().clone(), all.price_to_amount.as_mut()),
            (
                term.short.filter().clone(),
                term.short.price_to_amount.as_mut(),
            ),
            (
                term.long.filter().clone(),
                term.long.price_to_amount.as_mut(),
            ),
        ];

        // Process blocks that might cross a day boundary
        chain_state
            .iter()
            .filter(|block_state| {
                let age = (*prev_timestamp).saturating_sub(*block_state.timestamp);
                age % ONE_DAY_IN_SEC >= threshold
            })
            .for_each(|block_state| {
                let prev_days = prev_timestamp.difference_in_days_between(block_state.timestamp);
                let curr_days = timestamp.difference_in_days_between(block_state.timestamp);

                if prev_days == curr_days {
                    return;
                }

                // Update age_range cohort states
                age_cohorts.iter_mut().for_each(|(filter, state)| {
                    let is_now = filter.contains_time(curr_days);
                    let was_before = filter.contains_time(prev_days);

                    if is_now && !was_before {
                        state
                            .as_mut()
                            .unwrap()
                            .increment(&block_state.supply, block_state.price);
                    } else if was_before && !is_now {
                        state
                            .as_mut()
                            .unwrap()
                            .decrement(&block_state.supply, block_state.price);
                    }
                });

                // Update aggregate cohorts' price_to_amount
                if let Some(price) = block_state.price
                    && block_state.supply.value > Sats::ZERO
                {
                    aggregate_p2a.iter_mut().for_each(|(filter, p2a)| {
                        let is_now = filter.contains_time(curr_days);
                        let was_before = filter.contains_time(prev_days);

                        if is_now && !was_before {
                            p2a.um().increment(price, &block_state.supply);
                        } else if was_before && !is_now {
                            p2a.um().decrement(price, &block_state.supply);
                        }
                    });
                }
            });
    }
}
