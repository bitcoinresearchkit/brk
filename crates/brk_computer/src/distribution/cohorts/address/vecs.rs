use std::path::Path;

use brk_cohort::{CohortContext, Filter, Filtered};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, StoredF64, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, WritableVec, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, blocks,
    distribution::state::AddressCohortState,
    indexes,
    internal::ComputedFromHeightLast,
    prices,
};

use crate::distribution::metrics::{BasicCohortMetrics, CohortMetricsBase, ImportConfig, SupplyMetrics};

use super::super::traits::{CohortVecs, DynCohortVecs};

const VERSION: Version = Version::ZERO;

/// Address cohort with metrics and optional runtime state.
#[derive(Traversable)]
pub struct AddressCohortVecs<M: StorageMode = Rw> {
    /// Starting height when state was imported
    starting_height: Option<Height>,

    /// Runtime state for block-by-block processing
    #[traversable(skip)]
    pub state: Option<Box<AddressCohortState>>,

    /// Metric vectors
    #[traversable(flatten)]
    pub metrics: BasicCohortMetrics<M>,

    pub addr_count: ComputedFromHeightLast<StoredU64, M>,
    pub addr_count_30d_change: ComputedFromHeightLast<StoredF64, M>,
}

impl AddressCohortVecs {
    /// Import address cohort from database.
    ///
    /// `all_supply` is the supply metrics from the "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` ratios.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import(
        db: &Database,
        filter: Filter,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        states_path: Option<&Path>,
        all_supply: &SupplyMetrics,
    ) -> Result<Self> {
        let full_name = CohortContext::Address.full_name(&filter, name);

        let cfg = ImportConfig {
            db,
            filter,
            full_name: &full_name,
            context: CohortContext::Address,
            version,
            indexes,
            prices,
        };

        Ok(Self {
            starting_height: None,

            state: states_path
                .map(|path| Box::new(AddressCohortState::new(path, &full_name))),

            metrics: BasicCohortMetrics::forced_import(&cfg, all_supply)?,

            addr_count: ComputedFromHeightLast::forced_import(
                db,
                &cfg.name("addr_count"),
                version + VERSION,
                indexes,
            )?,
            addr_count_30d_change: ComputedFromHeightLast::forced_import(
                db,
                &cfg.name("addr_count_30d_change"),
                version + VERSION,
                indexes,
            )?,
        })
    }

    /// Reset starting height to zero.
    pub(crate) fn reset_starting_height(&mut self) {
        self.starting_height = Some(Height::ZERO);
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.addr_count.height as &mut dyn AnyStoredVec)
            .chain(self.metrics.par_iter_mut())
    }

    /// Commit state to disk (separate from vec writes for parallelization).
    pub(crate) fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.inner.write(height, cleanup)?;
        }
        Ok(())
    }
}

impl Filtered for AddressCohortVecs {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for AddressCohortVecs {
    fn min_stateful_height_len(&self) -> usize {
        self.addr_count
            .height
            .len()
            .min(self.metrics.min_stateful_height_len())
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_starting_height();
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        // Import state from runtime state if present
        if let Some(state) = self.state.as_mut() {
            // State files are saved AT height H, so to resume at H+1 we need to import at H
            // Decrement first, then increment result to match expected starting_height
            if let Some(mut prev_height) = starting_height.decremented() {
                // Import cost_basis_data state file (may adjust prev_height to actual file found)
                prev_height = state.inner.import_at_or_before(prev_height)?;

                // Restore supply state from height-indexed vectors
                state.inner.supply.value = self
                    .metrics
                    .supply
                    .total
                    .sats
                    .height
                    .collect_one(prev_height)
                    .unwrap();
                state.inner.supply.utxo_count = *self
                    .metrics
                    .outputs
                    .utxo_count
                    .height
                    .collect_one(prev_height)
                    .unwrap();
                state.addr_count = *self.addr_count.height.collect_one(prev_height).unwrap();

                // Restore realized cap from persisted exact values
                state.inner.restore_realized_cap();

                let result = prev_height.incremented();
                self.starting_height = Some(result);
                Ok(result)
            } else {
                // starting_height is 0, nothing to import
                self.starting_height = Some(Height::ZERO);
                Ok(Height::ZERO)
            }
        } else {
            self.starting_height = Some(starting_height);
            Ok(starting_height)
        }
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        use vecdb::WritableVec;
        self.addr_count
            .height
            .validate_computed_version_or_reset(base_version)?;
        self.metrics.validate_computed_versions(base_version)?;
        Ok(())
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.starting_height.is_some_and(|h| h > height) {
            return Ok(());
        }

        // Push addr_count from state
        if let Some(state) = self.state.as_ref() {
            self.addr_count
                .height
                .truncate_push(height, state.addr_count.into())?;
            self.metrics.truncate_push(height, &state.inner)?;
        }

        Ok(())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
    ) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            self.metrics.compute_then_truncate_push_unrealized_states(
                height,
                height_price,
                &mut state.inner,
            )?;
        }
        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;
        Ok(())
    }

    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_net_sentiment_height(starting_indexes, exit)
    }

    fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.inner.write(height, cleanup)?;
        }
        Ok(())
    }

    fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.inner.reset_cost_basis_data_if_needed()?;
        }
        Ok(())
    }

    fn reset_single_iteration_values(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.inner.reset_single_iteration_values();
        }
    }
}

impl CohortVecs for AddressCohortVecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.addr_count.height.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.addr_count.height)
                .collect::<Vec<_>>()
                .as_slice(),
            exit,
        )?;
        self.metrics.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.metrics).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_market_cap,
            exit,
        )?;
        Ok(())
    }
}
