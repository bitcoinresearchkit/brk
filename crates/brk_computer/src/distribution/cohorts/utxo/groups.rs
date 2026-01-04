use std::path::Path;

use brk_cohort::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, ByYear, Filter, Filtered, StateLevel, UTXOGroups,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Sats, Version};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, IterableVec};

use crate::{
    ComputeIndexes,
    distribution::DynCohortVecs,
    indexes,
    internal::{PERCENTILES, PERCENTILES_LEN},
    price,
};

use super::{super::traits::CohortVecs, vecs::UTXOCohortVecs};

const VERSION: Version = Version::new(0);

/// All UTXO cohorts organized by filter type.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct UTXOCohorts(pub(crate) UTXOGroups<UTXOCohortVecs>);

impl UTXOCohorts {
    /// Import all UTXO cohorts from database.
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
    ) -> Result<Self> {
        let v = version + VERSION;

        // Create "all" cohort first - it doesn't need global sources (it IS the global source)
        let all = UTXOCohortVecs::forced_import(
            db,
            Filter::All,
            "",
            version + VERSION + Version::ONE,
            indexes,
            price,
            states_path,
            StateLevel::PriceOnly,
            None,
        )?;

        // Get reference to all's supply for other cohorts to use as global source
        let all_supply = Some(&all.metrics.supply);

        // Create all cohorts first (while borrowing all_supply), then assemble struct
        let price_only = |f: Filter, name: &'static str| {
            UTXOCohortVecs::forced_import(
                db,
                f,
                name,
                v,
                indexes,
                price,
                states_path,
                StateLevel::PriceOnly,
                all_supply,
            )
        };

        let term = ByTerm::try_new(&price_only)?;

        let full = |f: Filter, name: &'static str| {
            UTXOCohortVecs::forced_import(
                db,
                f,
                name,
                v,
                indexes,
                price,
                states_path,
                StateLevel::Full,
                all_supply,
            )
        };
        let none = |f: Filter, name: &'static str| {
            UTXOCohortVecs::forced_import(
                db,
                f,
                name,
                v,
                indexes,
                price,
                states_path,
                StateLevel::None,
                all_supply,
            )
        };

        let epoch = ByEpoch::try_new(&full)?;
        let year = ByYear::try_new(&full)?;
        let type_ = BySpendableType::try_new(&full)?;
        let max_age = ByMaxAge::try_new(&none)?;
        let min_age = ByMinAge::try_new(&none)?;
        let age_range = ByAgeRange::try_new(&full)?;
        let amount_range = ByAmountRange::try_new(&full)?;
        let lt_amount = ByLowerThanAmount::try_new(&none)?;
        let ge_amount = ByGreatEqualAmount::try_new(&none)?;

        Ok(Self(UTXOGroups {
            all,
            term,
            epoch,
            year,
            type_,
            max_age,
            min_age,
            age_range,
            amount_range,
            lt_amount,
            ge_amount,
        }))
    }

    /// Compute overlapping cohorts from component age/amount range cohorts.
    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_age_range = &self.0.age_range;
        let by_amount_range = &self.0.amount_range;

        [(&mut self.0.all, by_age_range.iter().collect::<Vec<_>>())]
            .into_par_iter()
            .chain(self.0.min_age.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.max_age.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.term.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_age_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.ge_amount.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_amount_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .chain(self.0.lt_amount.par_iter_mut().map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_amount_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect::<Vec<_>>(),
                )
            }))
            .try_for_each(|(vecs, components)| {
                vecs.compute_from_stateful(starting_indexes, &components, exit)
            })
    }

    /// First phase of post-processing: compute index transforms.
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.par_iter_mut()
            .try_for_each(|v| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    /// Second phase of post-processing: compute relative metrics.
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2<HM, DM>(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: Option<&HM>,
        dateindex_to_market_cap: Option<&DM>,
        exit: &Exit,
    ) -> Result<()>
    where
        HM: IterableVec<Height, Dollars> + Sync,
        DM: IterableVec<DateIndex, Dollars> + Sync,
    {
        self.par_iter_mut().try_for_each(|v| {
            v.compute_rest_part2(
                indexes,
                price,
                starting_indexes,
                height_to_market_cap,
                dateindex_to_market_cap,
                exit,
            )
        })
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        // Collect all vecs from all cohorts (separate + aggregate)
        self.0
            .iter_mut()
            .flat_map(|v| v.par_iter_vecs_mut().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    /// Commit all states to disk (separate from vec writes for parallelization).
    pub fn commit_all_states(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_state(height, cleanup))
    }

    /// Get minimum height from all separate cohorts' height-indexed vectors.
    pub fn min_separate_stateful_height_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_stateful_height_len()))
            .min()
            .unwrap_or_default()
    }

    /// Get minimum dateindex from all separate cohorts' dateindex-indexed vectors.
    pub fn min_separate_stateful_dateindex_len(&self) -> usize {
        self.iter_separate()
            .map(|v| v.min_stateful_dateindex_len())
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Get minimum dateindex from all aggregate cohorts' dateindex-indexed vectors.
    /// This checks cost_basis percentiles which are only on aggregate cohorts.
    pub fn min_aggregate_stateful_dateindex_len(&self) -> usize {
        self.0
            .iter_aggregate()
            .filter_map(|v| v.metrics.cost_basis.as_ref())
            .filter_map(|cb| cb.percentiles.as_ref())
            .map(|cbp| cbp.min_stateful_dateindex_len())
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Import state for all separate cohorts at or before given height.
    /// Returns true if all imports succeeded and returned the expected height.
    pub fn import_separate_states(&mut self, height: Height) -> bool {
        self.par_iter_separate_mut()
            .map(|v| v.import_state(height).unwrap_or_default())
            .all(|h| h == height)
    }

    /// Reset state heights for all separate cohorts.
    pub fn reset_separate_state_heights(&mut self) {
        self.par_iter_separate_mut().for_each(|v| {
            v.reset_state_starting_height();
        });
    }

    /// Reset price_to_amount for all separate cohorts (called during fresh start).
    pub fn reset_separate_price_to_amount(&mut self) -> Result<()> {
        self.par_iter_separate_mut().try_for_each(|v| {
            if let Some(state) = v.state.as_mut() {
                state.reset_price_to_amount_if_needed()?;
            }
            Ok(())
        })
    }

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    /// Computes on-demand by merging age_range cohorts' price_to_amount data.
    /// This avoids maintaining redundant aggregate price_to_amount maps.
    pub fn truncate_push_aggregate_percentiles(&mut self, dateindex: DateIndex) -> Result<()> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        // Collect (filter, supply, price_to_amount as Vec) from age_range cohorts
        let age_range_data: Vec<_> = self
            .0
            .age_range
            .iter()
            .filter_map(|sub| {
                let state = sub.state.as_ref()?;
                let entries: Vec<(Dollars, Sats)> = state
                    .price_to_amount_iter()?
                    .map(|(p, &a)| (p, a))
                    .collect();
                Some((sub.filter().clone(), state.supply.value, entries))
            })
            .collect();

        // Compute percentiles for each aggregate filter
        for aggregate in self.0.iter_aggregate_mut() {
            let filter = aggregate.filter().clone();

            // Get cost_basis percentiles storage, skip if not configured
            let Some(percentiles) = aggregate
                .metrics
                .cost_basis
                .as_mut()
                .and_then(|cb| cb.percentiles.as_mut())
            else {
                continue;
            };

            // Collect relevant cohort data for this aggregate
            let relevant: Vec<_> = age_range_data
                .iter()
                .filter(|(sub_filter, _, _)| filter.includes(sub_filter))
                .collect();

            // Calculate total supply
            let total_supply: u64 = relevant.iter().map(|(_, s, _)| u64::from(*s)).sum();

            if total_supply == 0 {
                percentiles.truncate_push(dateindex, &[Dollars::NAN; PERCENTILES_LEN])?;
                continue;
            }

            // K-way merge using min-heap: O(n log k) where k = number of cohorts
            // Each heap entry: (price, amount, cohort_idx, entry_idx)
            let mut heap: BinaryHeap<Reverse<(Dollars, usize, usize)>> = BinaryHeap::new();

            // Initialize heap with first entry from each cohort
            for (cohort_idx, (_, _, entries)) in relevant.iter().enumerate() {
                if !entries.is_empty() {
                    heap.push(Reverse((entries[0].0, cohort_idx, 0)));
                }
            }

            let targets = PERCENTILES.map(|p| total_supply * u64::from(p) / 100);
            let mut result = [Dollars::NAN; PERCENTILES_LEN];
            let mut accumulated = 0u64;
            let mut pct_idx = 0;
            let mut current_price: Option<Dollars> = None;
            let mut amount_at_price = 0u64;

            while let Some(Reverse((price, cohort_idx, entry_idx))) = heap.pop() {
                let (_, _, entries) = relevant[cohort_idx];
                let (_, amount) = entries[entry_idx];

                // If price changed, finalize previous price
                if let Some(current_price) = current_price
                    && current_price != price
                {
                    accumulated += amount_at_price;

                    while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                        result[pct_idx] = current_price;
                        pct_idx += 1;
                    }

                    if pct_idx >= PERCENTILES_LEN {
                        break;
                    }

                    amount_at_price = 0;
                }

                current_price = Some(price);
                amount_at_price += u64::from(amount);

                // Push next entry from this cohort
                let next_idx = entry_idx + 1;
                if next_idx < entries.len() {
                    heap.push(Reverse((entries[next_idx].0, cohort_idx, next_idx)));
                }
            }

            // Finalize last price
            if let Some(price) = current_price {
                accumulated += amount_at_price;
                while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                    result[pct_idx] = price;
                    pct_idx += 1;
                }
            }

            percentiles.truncate_push(dateindex, &result)?;
        }

        Ok(())
    }

    /// Validate computed versions for all cohorts (separate and aggregate).
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        // Validate separate cohorts
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))?;

        // Validate aggregate cohorts' cost_basis percentiles
        for v in self.0.iter_aggregate_mut() {
            v.validate_computed_versions(base_version)?;
        }

        Ok(())
    }
}
