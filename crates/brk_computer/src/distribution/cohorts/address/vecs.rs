use std::path::Path;

use brk_cohort::{CohortContext, Filter, Filtered};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, IterableVec};

use crate::{
    ComputeIndexes,
    distribution::state::AddressCohortState,
    indexes,
    internal::ComputedBlockLast,
    price,
};

use crate::distribution::metrics::{CohortMetrics, ImportConfig, SupplyMetrics};

use super::super::traits::{CohortVecs, DynCohortVecs};

const VERSION: Version = Version::ZERO;

/// Address cohort with metrics and optional runtime state.
#[derive(Clone, Traversable)]
pub struct AddressCohortVecs {
    /// Starting height when state was imported
    starting_height: Option<Height>,

    /// Runtime state for block-by-block processing
    #[traversable(skip)]
    pub state: Option<AddressCohortState>,

    /// Metric vectors
    #[traversable(flatten)]
    pub metrics: CohortMetrics,

    pub addr_count: ComputedBlockLast<StoredU64>,
}

impl AddressCohortVecs {
    /// Import address cohort from database.
    ///
    /// `all_supply` is the supply metrics from the "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` ratios. Pass `None` if not available.
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        filter: Filter,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: Option<&Path>,
        all_supply: Option<&SupplyMetrics>,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();
        let full_name = CohortContext::Address.full_name(&filter, name);

        let cfg = ImportConfig {
            db,
            filter,
            full_name: &full_name,
            context: CohortContext::Address,
            version,
            indexes,
            price,
            up_to_1h_realized: None,
        };

        Ok(Self {
            starting_height: None,

            state: states_path
                .map(|path| AddressCohortState::new(path, &full_name, compute_dollars)),

            metrics: CohortMetrics::forced_import(&cfg, all_supply)?,

            addr_count: ComputedBlockLast::forced_import(
                db,
                &cfg.name("addr_count"),
                version + VERSION,
                indexes,
            )?,
        })
    }

    /// Get the starting height when state was imported.
    pub fn starting_height(&self) -> Option<Height> {
        self.starting_height
    }

    /// Set the starting height.
    pub fn set_starting_height(&mut self, height: Height) {
        self.starting_height = Some(height);
    }

    /// Reset starting height to zero.
    pub fn reset_starting_height(&mut self) {
        self.starting_height = Some(Height::ZERO);
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.addr_count.height as &mut dyn AnyStoredVec)
            .chain(self.metrics.par_iter_mut())
    }

    /// Commit state to disk (separate from vec writes for parallelization).
    pub fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
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

    fn min_stateful_dateindex_len(&self) -> usize {
        self.metrics.min_stateful_dateindex_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_starting_height();
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        use vecdb::GenericStoredVec;

        // Import state from runtime state if present
        if let Some(state) = self.state.as_mut() {
            // State files are saved AT height H, so to resume at H+1 we need to import at H
            // Decrement first, then increment result to match expected starting_height
            if let Some(mut prev_height) = starting_height.decremented() {
                // Import price_to_amount state file (may adjust prev_height to actual file found)
                prev_height = state.inner.import_at_or_before(prev_height)?;

                // Restore supply state from height-indexed vectors
                state.inner.supply.value = self
                    .metrics
                    .supply
                    .supply
                    .sats
                    .height
                    .read_once(prev_height)?;
                state.inner.supply.utxo_count = *self
                    .metrics
                    .outputs
                    .utxo_count
                    .height
                    .read_once(prev_height)?;
                state.addr_count = *self.addr_count.height.read_once(prev_height)?;

                // Restore realized cap if present
                if let Some(realized_metrics) = self.metrics.realized.as_mut()
                    && let Some(realized_state) = state.inner.realized.as_mut()
                {
                    realized_state.cap = realized_metrics
                        .realized_cap
                        .height
                        .read_once(prev_height)?;
                }

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
        use vecdb::GenericStoredVec;
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
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
    ) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            self.metrics.compute_then_truncate_push_unrealized_states(
                height,
                height_price,
                dateindex,
                date_price,
                &mut state.inner,
            )?;
        }
        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.addr_count
            .compute_rest(indexes, starting_indexes, exit)?;
        self.metrics
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;
        Ok(())
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
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_market_cap,
            dateindex_to_market_cap,
            exit,
        )?;
        Ok(())
    }
}
