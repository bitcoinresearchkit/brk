use std::path::Path;

use brk_cohort::{
    AddressGroups, ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, Filter, Filtered,
};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Version};
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, distribution::DynCohortVecs, indexes, prices};

use crate::distribution::metrics::{CohortMetricsBase, SupplyMetrics};

use super::{super::traits::CohortVecs, vecs::AddressCohortVecs};

const VERSION: Version = Version::new(0);

/// All Address cohorts organized by filter type.
#[derive(Deref, DerefMut, Traversable)]
pub struct AddressCohorts<M: StorageMode = Rw>(AddressGroups<AddressCohortVecs<M>>);

impl AddressCohorts {
    /// Import all Address cohorts from database.
    ///
    /// `all_supply` is the supply metrics from the UTXO "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` ratios.
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        states_path: &Path,
        all_supply: &SupplyMetrics,
    ) -> Result<Self> {
        let v = version + VERSION;

        // Helper to create a cohort - only amount_range cohorts have state
        let create = |filter: Filter,
                      name: &'static str,
                      has_state: bool|
         -> Result<AddressCohortVecs> {
            let sp = if has_state { Some(states_path) } else { None };
            AddressCohortVecs::forced_import(db, filter, name, v, indexes, prices, sp, all_supply)
        };

        let full = |f: Filter, name: &'static str| create(f, name, true);
        let none = |f: Filter, name: &'static str| create(f, name, false);

        Ok(Self(AddressGroups {
            amount_range: ByAmountRange::try_new(&full)?,
            lt_amount: ByLowerThanAmount::try_new(&none)?,
            ge_amount: ByGreatEqualAmount::try_new(&none)?,
        }))
    }

    /// Apply a function to each aggregate cohort with its source cohorts (in parallel).
    fn for_each_aggregate<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn(&mut AddressCohortVecs, Vec<&AddressCohortVecs>) -> Result<()> + Sync,
    {
        let by_amount_range = &self.0.amount_range;

        let pairs: Vec<_> = self
            .0
            .ge_amount
            .iter_mut()
            .chain(self.0.lt_amount.iter_mut())
            .map(|vecs| {
                let filter = vecs.filter().clone();
                (
                    vecs,
                    by_amount_range
                        .iter()
                        .filter(|other| filter.includes(other.filter()))
                        .collect(),
                )
            })
            .collect();

        pairs
            .into_par_iter()
            .try_for_each(|(vecs, sources)| f(vecs, sources))
    }

    /// Compute overlapping cohorts from component amount_range cohorts.
    pub(crate) fn compute_overlapping_vecs(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.for_each_aggregate(|vecs, sources| {
            vecs.compute_from_stateful(starting_indexes, &sources, exit)
        })
    }

    /// First phase of post-processing: compute index transforms.
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute addr_count_30d_change using rolling window
        self.par_iter_mut().try_for_each(|v| {
            v.addr_count_30d_change.height.compute_rolling_change(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &v.addr_count.height,
                exit,
            )
        })?;

        // 2. Compute all metrics except net_sentiment
        self.par_iter_mut()
            .try_for_each(|v| v.compute_rest_part1(blocks, prices, starting_indexes, exit))?;

        // 2. Compute net_sentiment.height for separate cohorts (greed - pain)
        self.par_iter_separate_mut().try_for_each(|v| {
            v.metrics
                .compute_net_sentiment_height(starting_indexes, exit)
        })?;

        // 3. Compute net_sentiment.height for aggregate cohorts (weighted average)
        self.for_each_aggregate(|vecs, sources| {
            let metrics: Vec<_> = sources.iter().map(|v| &v.metrics).collect();
            vecs.metrics
                .compute_net_sentiment_from_others(starting_indexes, &metrics, exit)
        })?;

        Ok(())
    }

    /// Second phase of post-processing: compute relative metrics.
    pub(crate) fn compute_rest_part2<HM>(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &HM,
        exit: &Exit,
    ) -> Result<()>
    where
        HM: ReadableVec<Height, Dollars> + Sync,
    {
        self.0.par_iter_mut().try_for_each(|v| {
            v.compute_rest_part2(
                blocks,
                prices,
                starting_indexes,
                height_to_market_cap,
                exit,
            )
        })
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        // Collect all vecs from all cohorts
        self.0
            .iter_mut()
            .flat_map(|v| v.par_iter_vecs_mut().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    /// Commit all states to disk (separate from vec writes for parallelization).
    pub(crate) fn commit_all_states(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.write_state(height, cleanup))
    }

    /// Get minimum height from all separate cohorts' height-indexed vectors.
    pub(crate) fn min_separate_stateful_height_len(&self) -> Height {
        self.iter_separate()
            .map(|v| Height::from(v.min_stateful_height_len()))
            .min()
            .unwrap_or_default()
    }

    /// Import state for all separate cohorts at or before given height.
    /// Returns true if all imports succeeded and returned the expected height.
    pub(crate) fn import_separate_states(&mut self, height: Height) -> bool {
        self.par_iter_separate_mut()
            .map(|v| v.import_state(height).unwrap_or_default())
            .all(|h| h == height)
    }

    /// Reset state heights for all separate cohorts.
    pub(crate) fn reset_separate_state_heights(&mut self) {
        self.par_iter_separate_mut().for_each(|v| {
            v.reset_state_starting_height();
        });
    }

    /// Reset cost_basis_data for all separate cohorts (called during fresh start).
    pub(crate) fn reset_separate_cost_basis_data(&mut self) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.reset_cost_basis_data_if_needed())
    }

    /// Validate computed versions for all separate cohorts.
    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.par_iter_separate_mut()
            .try_for_each(|v| v.validate_computed_versions(base_version))
    }
}
