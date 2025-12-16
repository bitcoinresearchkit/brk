//! Address cohort vectors with metrics and state.

use std::path::Path;

use brk_error::Result;
use brk_grouper::{CohortContext, Filter, Filtered};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, StoredU64, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableVec,
    PcoVec,
};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes, price,
    states::AddressCohortState,
};

use super::super::metrics::{CohortMetrics, ImportConfig};
use super::traits::{CohortVecs, DynCohortVecs};

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

    /// Address count at each height
    pub height_to_addr_count: EagerVec<PcoVec<Height, StoredU64>>,

    /// Address count indexed by various dimensions
    pub indexes_to_addr_count: ComputedVecsFromHeight<StoredU64>,
}

impl AddressCohortVecs {
    /// Import address cohort from database.
    pub fn forced_import(
        db: &Database,
        filter: Filter,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: Option<&Path>,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();
        let full_name = filter.to_full_name(CohortContext::Address);

        let cfg = ImportConfig {
            db,
            filter,
            context: CohortContext::Address,
            version,
            indexes,
            price,
        };

        Ok(Self {
            starting_height: None,

            state: states_path
                .map(|path| AddressCohortState::new(path, &full_name, compute_dollars)),

            metrics: CohortMetrics::forced_import(&cfg)?,

            height_to_addr_count: EagerVec::forced_import(
                db,
                &cfg.name("addr_count"),
                version + VERSION + Version::ZERO,
            )?,

            indexes_to_addr_count: ComputedVecsFromHeight::forced_import(
                db,
                &cfg.name("addr_count"),
                Source::None,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
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

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.height_to_addr_count
            .len()
            .min(self.metrics.supply.min_len())
            .min(self.metrics.activity.min_len())
    }
}

impl Filtered for AddressCohortVecs {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for AddressCohortVecs {
    fn min_height_vecs_len(&self) -> usize {
        self.min_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_starting_height();
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        // Import state from runtime state if present
        if let Some(state) = self.state.as_mut() {
            let imported = state.inner.import_at_or_before(starting_height)?;
            self.starting_height = Some(imported);

            // Restore addr_count from last known value
            if let Some(prev_height) = imported.decremented() {
                use vecdb::TypedVecIterator;
                state.addr_count = *self
                    .height_to_addr_count
                    .into_iter()
                    .get_unwrap(prev_height);
            }

            Ok(imported)
        } else {
            self.starting_height = Some(starting_height);
            Ok(starting_height)
        }
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        use vecdb::GenericStoredVec;
        self.height_to_addr_count
            .validate_computed_version_or_reset(
                base_version + self.height_to_addr_count.inner_version(),
            )?;
        self.metrics.validate_computed_versions(base_version)?;
        Ok(())
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.starting_height.is_some_and(|h| h > height) {
            return Ok(());
        }

        // Push addr_count from state
        if let Some(state) = self.state.as_ref() {
            self.height_to_addr_count
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
        if let Some(state) = self.state.as_ref() {
            self.metrics.compute_then_truncate_push_unrealized_states(
                height,
                height_price,
                dateindex,
                date_price,
                &state.inner,
            )?;
        }
        Ok(())
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.height_to_addr_count.safe_write(exit)?;
        self.metrics.safe_flush(exit)?;

        if let Some(state) = self.state.as_mut() {
            state.inner.commit(height)?;
        }

        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_addr_count.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_addr_count),
        )?;
        self.metrics
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;
        Ok(())
    }
}

impl CohortVecs for AddressCohortVecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_addr_count.compute_sum_of_others(
            starting_indexes.height,
            others
                .iter()
                .map(|v| &v.height_to_addr_count)
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
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply,
            height_to_market_cap,
            dateindex_to_market_cap,
            height_to_realized_cap,
            dateindex_to_realized_cap,
            exit,
        )?;
        Ok(())
    }
}
