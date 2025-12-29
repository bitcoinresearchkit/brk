use std::path::Path;

use brk_error::Result;
use brk_grouper::{
    AddressGroups, ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, Filter, Filtered,
};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use derive_deref::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, IterableVec};

use crate::{Indexes, indexes, price, stateful::DynCohortVecs};

use super::{super::metrics::SupplyMetrics, AddressCohortVecs, CohortVecs};

const VERSION: Version = Version::new(0);

/// All Address cohorts organized by filter type.
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct AddressCohorts(AddressGroups<AddressCohortVecs>);

impl AddressCohorts {
    /// Import all Address cohorts from database.
    ///
    /// `all_supply` is the supply metrics from the UTXO "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` ratios.
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
        all_supply: Option<&SupplyMetrics>,
    ) -> Result<Self> {
        let v = version + VERSION + Version::ZERO;

        // Helper to create a cohort - only amount_range cohorts have state
        let create =
            |filter: Filter, name: &'static str, has_state: bool| -> Result<AddressCohortVecs> {
                let sp = if has_state { Some(states_path) } else { None };
                AddressCohortVecs::forced_import(db, filter, name, v, indexes, price, sp, all_supply)
            };

        let full = |f: Filter, name: &'static str| create(f, name, true);
        let none = |f: Filter, name: &'static str| create(f, name, false);

        Ok(Self(AddressGroups {
            amount_range: ByAmountRange::try_new(&full)?,
            lt_amount: ByLowerThanAmount::try_new(&none)?,
            ge_amount: ByGreatEqualAmount::try_new(&none)?,
        }))
    }

    /// Compute overlapping cohorts from component amount_range cohorts.
    ///
    /// For example, ">=1 BTC" cohort is computed from sum of amount_range cohorts that match.
    pub fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let by_amount_range = &self.0.amount_range;

        // ge_amount cohorts computed from matching amount_range cohorts
        [
            self.0
                .ge_amount
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_amount_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            // lt_amount cohorts computed from matching amount_range cohorts
            self.0
                .lt_amount
                .par_iter_mut()
                .map(|vecs| {
                    let filter = vecs.filter().clone();
                    (
                        vecs,
                        by_amount_range
                            .iter()
                            .filter(|other| filter.includes(other.filter()))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        .try_for_each(|(vecs, components)| {
            vecs.compute_from_stateful(starting_indexes, &components, exit)
        })
    }

    /// First phase of post-processing: compute index transforms.
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.par_iter_mut()
            .try_for_each(|v| v.compute_rest_part1(indexes, price, starting_indexes, exit))
    }

    /// Second phase of post-processing: compute relative metrics.
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2<S, HM, DM>(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &S,
        height_to_market_cap: Option<&HM>,
        dateindex_to_market_cap: Option<&DM>,
        exit: &Exit,
    ) -> Result<()>
    where
        S: IterableVec<Height, Bitcoin> + Sync,
        HM: IterableVec<Height, Dollars> + Sync,
        DM: IterableVec<DateIndex, Dollars> + Sync,
    {
        self.0.par_iter_mut().try_for_each(|v| {
            v.compute_rest_part2(
                indexes,
                price,
                starting_indexes,
                height_to_supply,
                height_to_market_cap,
                dateindex_to_market_cap,
                exit,
            )
        })
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        // Collect all vecs from all cohorts
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
    pub fn min_separate_height_vecs_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_height_vecs_len()))
            .min()
            .unwrap_or_default()
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

    /// Validate computed versions for all separate cohorts.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))
    }
}
