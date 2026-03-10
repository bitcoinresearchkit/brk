use std::path::Path;

use brk_cohort::{CohortContext, Filter, Filtered};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredI64, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::state::{AddressCohortState, MinimalRealizedState},
    indexes,
    internal::{ComputedPerBlock, RollingDelta1m},
    prices,
};

use crate::distribution::metrics::{ImportConfig, MinimalCohortMetrics};

use super::super::traits::{CohortVecs, DynCohortVecs};
#[derive(Traversable)]
pub struct AddressCohortVecs<M: StorageMode = Rw> {
    starting_height: Option<Height>,

    #[traversable(skip)]
    pub state: Option<Box<AddressCohortState<MinimalRealizedState>>>,

    #[traversable(flatten)]
    pub metrics: MinimalCohortMetrics<M>,

    pub addr_count: ComputedPerBlock<StoredU64, M>,
    #[traversable(wrap = "addr_count", rename = "delta")]
    pub addr_count_delta: RollingDelta1m<StoredU64, StoredI64, M>,
}

impl AddressCohortVecs {
    pub(crate) fn forced_import(
        db: &Database,
        filter: Filter,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        states_path: Option<&Path>,
    ) -> Result<Self> {
        let full_name = CohortContext::Address.full_name(&filter, name);

        let cfg = ImportConfig {
            db,
            filter: &filter,
            full_name: &full_name,
            version,
            indexes,
        };

        Ok(Self {
            starting_height: None,

            state: states_path.map(|path| Box::new(AddressCohortState::new(path, &full_name))),

            metrics: MinimalCohortMetrics::forced_import(&cfg)?,

            addr_count: ComputedPerBlock::forced_import(
                db,
                &cfg.name("addr_count"),
                version,
                indexes,
            )?,
            addr_count_delta: RollingDelta1m::forced_import(
                db,
                &cfg.name("addr_count_delta"),
                version + Version::ONE,
                indexes,
            )?,
        })
    }

    pub(crate) fn reset_starting_height(&mut self) {
        self.starting_height = Some(Height::ZERO);
    }

    pub(crate) fn par_iter_vecs_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.push(&mut self.addr_count.height as &mut dyn AnyStoredVec);
        vecs.extend(self.metrics.collect_all_vecs_mut());
        vecs.into_par_iter()
    }

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
    fn min_stateful_len(&self) -> usize {
        self.addr_count
            .height
            .len()
            .min(self.metrics.min_stateful_len())
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_starting_height();
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        if let Some(state) = self.state.as_mut() {
            if let Some(mut prev_height) = starting_height.decremented() {
                prev_height = state.inner.import_at_or_before(prev_height)?;

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

                state.inner.restore_realized_cap();

                let result = prev_height.incremented();
                self.starting_height = Some(result);
                Ok(result)
            } else {
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
        Ok(())
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.starting_height.is_some_and(|h| h > height) {
            return Ok(());
        }

        if let Some(state) = self.state.as_ref() {
            self.addr_count
                .height
                .truncate_push(height, state.addr_count.into())?;
            self.metrics.supply.truncate_push(height, &state.inner)?;
            self.metrics.outputs.truncate_push(height, &state.inner)?;
            self.metrics.realized.truncate_push(height, &state.inner)?;
        }

        Ok(())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        _height: Height,
        _height_price: Cents,
        _is_day_boundary: bool,
    ) -> Result<()> {
        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(blocks, prices, starting_indexes, exit)
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
        starting_indexes: &Indexes,
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
        self.metrics.compute_from_sources(
            starting_indexes,
            &others.iter().map(|v| &v.metrics).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part2(prices, starting_indexes, exit)
    }
}
