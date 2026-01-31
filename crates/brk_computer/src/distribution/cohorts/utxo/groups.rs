use std::{cmp::Reverse, collections::BinaryHeap, path::Path};

use brk_cohort::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, ByYear, Filter, Filtered, StateLevel, UTXOGroups,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, DateIndex, Dollars, Height, Sats, StoredF32, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, GenericStoredVec, IterableVec};

use crate::{
    ComputeIndexes,
    distribution::DynCohortVecs,
    indexes,
    internal::{PERCENTILES, PERCENTILES_LEN, compute_spot_percentile_rank},
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

        // Phase 1: Import base cohorts that don't need adjusted (age_range, amount_range, etc.)
        // These are the source cohorts for overlapping computations.
        let base = |f: Filter, name: &'static str| {
            UTXOCohortVecs::forced_import(
                db,
                f,
                name,
                v,
                indexes,
                price,
                states_path,
                StateLevel::Full,
                None,
                None,
            )
        };

        let age_range = ByAgeRange::try_new(&base)?;
        let amount_range = ByAmountRange::try_new(&base)?;
        let epoch = ByEpoch::try_new(&base)?;
        let year = ByYear::try_new(&base)?;
        let type_ = BySpendableType::try_new(&base)?;

        // Get up_to_1h realized for adjusted computation (cohort - up_to_1h)
        let up_to_1h_realized = age_range.up_to_1h.metrics.realized.as_ref();

        // Phase 2: Import "all" cohort (needs up_to_1h for adjusted, is global supply source)
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
            up_to_1h_realized,
        )?;

        let all_supply = Some(&all.metrics.supply);

        // Phase 3: Import cohorts that need adjusted and/or all_supply
        let price_only_adjusted = |f: Filter, name: &'static str| {
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
                up_to_1h_realized,
            )
        };

        let term = ByTerm::try_new(&price_only_adjusted)?;

        let none_adjusted = |f: Filter, name: &'static str| {
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
                up_to_1h_realized,
            )
        };

        let max_age = ByMaxAge::try_new(&none_adjusted)?;

        // Phase 4: Import remaining cohorts (no adjusted needed)
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
                None,
            )
        };

        let min_age = ByMinAge::try_new(&none)?;
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
    /// This checks cost_basis metrics which are only on aggregate cohorts.
    pub fn min_aggregate_stateful_dateindex_len(&self) -> usize {
        self.0
            .iter_aggregate()
            .filter_map(|v| v.metrics.cost_basis.as_ref())
            .map(|cb| cb.min_stateful_dateindex_len())
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

    /// Reset cost_basis_data for all separate cohorts (called during fresh start).
    pub fn reset_separate_cost_basis_data(&mut self) -> Result<()> {
        self.par_iter_separate_mut().try_for_each(|v| {
            if let Some(state) = v.state.as_mut() {
                state.reset_cost_basis_data_if_needed()?;
            }
            Ok(())
        })
    }

    /// Compute and push percentiles for aggregate cohorts (all, sth, lth).
    /// Computes on-demand by merging age_range cohorts' cost_basis_data data.
    /// This avoids maintaining redundant aggregate cost_basis_data maps.
    /// Computes both sat-weighted (percentiles) and USD-weighted (invested_capital) percentiles.
    pub fn truncate_push_aggregate_percentiles(
        &mut self,
        dateindex: DateIndex,
        spot: Dollars,
    ) -> Result<()> {
        // Collect (filter, entries, total_sats, total_usd) from age_range cohorts.
        // Keep data in CentsUnsigned to avoid float conversions until output.
        // Compute totals during collection to avoid a second pass.
        let age_range_data: Vec<_> = self
            .0
            .age_range
            .iter()
            .filter_map(|sub| {
                let state = sub.state.as_ref()?;
                let mut total_sats: u64 = 0;
                let mut total_usd: u128 = 0;
                let entries: Vec<(CentsUnsigned, Sats)> = state
                    .cost_basis_data_iter()?
                    .map(|(price, &sats)| {
                        let sats_u64 = u64::from(sats);
                        let price_u128 = price.as_u128();
                        total_sats += sats_u64;
                        total_usd += price_u128 * sats_u64 as u128;
                        (price, sats)
                    })
                    .collect();
                Some((sub.filter().clone(), entries, total_sats, total_usd))
            })
            .collect();

        // Compute percentiles for each aggregate filter
        for aggregate in self.0.iter_aggregate_mut() {
            let filter = aggregate.filter().clone();

            // Get cost_basis, skip if not configured
            let Some(cost_basis) = aggregate.metrics.cost_basis.as_mut() else {
                continue;
            };

            // Collect relevant cohort data for this aggregate and sum totals
            let mut total_sats: u64 = 0;
            let mut total_usd: u128 = 0;
            let relevant: Vec<_> = age_range_data
                .iter()
                .filter(|(sub_filter, _, _, _)| filter.includes(sub_filter))
                .map(|(_, entries, cohort_sats, cohort_usd)| {
                    total_sats += cohort_sats;
                    total_usd += cohort_usd;
                    entries
                })
                .collect();

            if total_sats == 0 {
                let nan_prices = [Dollars::NAN; PERCENTILES_LEN];
                if let Some(percentiles) = cost_basis.percentiles.as_mut() {
                    percentiles.truncate_push(dateindex, &nan_prices)?;
                }
                if let Some(invested_capital) = cost_basis.invested_capital.as_mut() {
                    invested_capital.truncate_push(dateindex, &nan_prices)?;
                }
                if let Some(spot_pct) = cost_basis.spot_cost_basis_percentile.as_mut() {
                    spot_pct
                        .dateindex
                        .truncate_push(dateindex, StoredF32::NAN)?;
                }
                if let Some(spot_pct) = cost_basis.spot_invested_capital_percentile.as_mut() {
                    spot_pct
                        .dateindex
                        .truncate_push(dateindex, StoredF32::NAN)?;
                }
                continue;
            }

            // K-way merge using min-heap: O(n log k) where k = number of cohorts
            let mut heap: BinaryHeap<Reverse<(CentsUnsigned, usize, usize)>> = BinaryHeap::new();

            // Initialize heap with first entry from each cohort
            for (cohort_idx, entries) in relevant.iter().enumerate() {
                if !entries.is_empty() {
                    heap.push(Reverse((entries[0].0, cohort_idx, 0)));
                }
            }

            // Compute both sat-weighted and USD-weighted percentiles in one pass
            let sat_targets = PERCENTILES.map(|p| total_sats * u64::from(p) / 100);
            let usd_targets = PERCENTILES.map(|p| total_usd * u128::from(p) / 100);

            let mut sat_result = [Dollars::NAN; PERCENTILES_LEN];
            let mut usd_result = [Dollars::NAN; PERCENTILES_LEN];

            let mut cumsum_sats: u64 = 0;
            let mut cumsum_usd: u128 = 0;
            let mut sat_idx = 0;
            let mut usd_idx = 0;

            let mut current_price: Option<CentsUnsigned> = None;
            let mut sats_at_price: u64 = 0;
            let mut usd_at_price: u128 = 0;

            while let Some(Reverse((price, cohort_idx, entry_idx))) = heap.pop() {
                let entries = relevant[cohort_idx];
                let (_, amount) = entries[entry_idx];
                let amount_u64 = u64::from(amount);
                let price_u128 = price.as_u128();

                // If price changed, finalize previous price
                if let Some(prev_price) = current_price
                    && prev_price != price
                {
                    cumsum_sats += sats_at_price;
                    cumsum_usd += usd_at_price;

                    // Only convert to dollars if we still need percentiles
                    if sat_idx < PERCENTILES_LEN || usd_idx < PERCENTILES_LEN {
                        let prev_dollars = prev_price.to_dollars();
                        while sat_idx < PERCENTILES_LEN && cumsum_sats >= sat_targets[sat_idx] {
                            sat_result[sat_idx] = prev_dollars;
                            sat_idx += 1;
                        }
                        while usd_idx < PERCENTILES_LEN && cumsum_usd >= usd_targets[usd_idx] {
                            usd_result[usd_idx] = prev_dollars;
                            usd_idx += 1;
                        }

                        // Early exit if all percentiles found
                        if sat_idx >= PERCENTILES_LEN && usd_idx >= PERCENTILES_LEN {
                            break;
                        }
                    }

                    sats_at_price = 0;
                    usd_at_price = 0;
                }

                current_price = Some(price);
                sats_at_price += amount_u64;
                usd_at_price += price_u128 * amount_u64 as u128;

                // Push next entry from this cohort
                let next_idx = entry_idx + 1;
                if next_idx < entries.len() {
                    heap.push(Reverse((entries[next_idx].0, cohort_idx, next_idx)));
                }
            }

            // Finalize last price (skip if we already found all percentiles via early exit)
            if (sat_idx < PERCENTILES_LEN || usd_idx < PERCENTILES_LEN)
                && let Some(price) = current_price
            {
                cumsum_sats += sats_at_price;
                cumsum_usd += usd_at_price;

                let price_dollars = price.to_dollars();
                while sat_idx < PERCENTILES_LEN && cumsum_sats >= sat_targets[sat_idx] {
                    sat_result[sat_idx] = price_dollars;
                    sat_idx += 1;
                }
                while usd_idx < PERCENTILES_LEN && cumsum_usd >= usd_targets[usd_idx] {
                    usd_result[usd_idx] = price_dollars;
                    usd_idx += 1;
                }
            }

            // Push both sat-weighted and USD-weighted results
            if let Some(percentiles) = cost_basis.percentiles.as_mut() {
                percentiles.truncate_push(dateindex, &sat_result)?;
            }
            if let Some(invested_capital) = cost_basis.invested_capital.as_mut() {
                invested_capital.truncate_push(dateindex, &usd_result)?;
            }

            // Compute and push spot percentile ranks
            if let Some(spot_pct) = cost_basis.spot_cost_basis_percentile.as_mut() {
                let rank = compute_spot_percentile_rank(&sat_result, spot);
                spot_pct.dateindex.truncate_push(dateindex, rank)?;
            }
            if let Some(spot_pct) = cost_basis.spot_invested_capital_percentile.as_mut() {
                let rank = compute_spot_percentile_rank(&usd_result, spot);
                spot_pct.dateindex.truncate_push(dateindex, rank)?;
            }
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
