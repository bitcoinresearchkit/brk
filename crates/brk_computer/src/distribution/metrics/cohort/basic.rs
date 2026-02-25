use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, distribution::state::CohortState, prices};

use crate::distribution::metrics::{
    ActivityMetrics, CohortMetricsBase, CostBasisBase, ImportConfig, OutputsMetrics, RealizedBase,
    RelativeWithRelToAll, SupplyMetrics, UnrealizedBase,
};

/// Basic cohort metrics: no extensions, with relative (rel_to_all).
/// Used by: epoch, year, type (spendable), amount, address cohorts.
#[derive(Traversable)]
pub struct BasicCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityMetrics<M>>,
    pub realized: Box<RealizedBase<M>>,
    pub cost_basis: Box<CostBasisBase<M>>,
    pub unrealized: Box<UnrealizedBase<M>>,
    pub relative: Box<RelativeWithRelToAll>,
}

impl CohortMetricsBase for BasicCohortMetrics {
    fn filter(&self) -> &Filter { &self.filter }
    fn supply(&self) -> &SupplyMetrics { &self.supply }
    fn supply_mut(&mut self) -> &mut SupplyMetrics { &mut self.supply }
    fn outputs(&self) -> &OutputsMetrics { &self.outputs }
    fn outputs_mut(&mut self) -> &mut OutputsMetrics { &mut self.outputs }
    fn activity(&self) -> &ActivityMetrics { &self.activity }
    fn activity_mut(&mut self) -> &mut ActivityMetrics { &mut self.activity }
    fn realized_base(&self) -> &RealizedBase { &self.realized }
    fn realized_base_mut(&mut self) -> &mut RealizedBase { &mut self.realized }
    fn unrealized_base(&self) -> &UnrealizedBase { &self.unrealized }
    fn unrealized_base_mut(&mut self) -> &mut UnrealizedBase { &mut self.unrealized }
    fn cost_basis_base(&self) -> &CostBasisBase { &self.cost_basis }
    fn cost_basis_base_mut(&mut self) -> &mut CostBasisBase { &mut self.cost_basis }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;
        Ok(())
    }
    fn compute_then_truncate_push_unrealized_states(
        &mut self, height: Height, height_price: Cents, state: &mut CohortState,
    ) -> Result<()> {
        state.apply_pending();
        self.cost_basis.truncate_push_minmax(height, state)?;
        let (height_unrealized_state, _) = state.compute_unrealized_states(height_price, None);
        self.unrealized.truncate_push(height, &height_unrealized_state)?;
        Ok(())
    }
    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.outputs.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.cost_basis.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }
}

impl BasicCohortMetrics {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        all_supply: &SupplyMetrics,
    ) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let unrealized = UnrealizedBase::forced_import(cfg)?;
        let realized = RealizedBase::forced_import(cfg)?;

        let relative = RelativeWithRelToAll::forced_import(
            cfg, &unrealized, &supply, all_supply, &realized,
        );

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityMetrics::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisBase::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
        })
    }

    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.collect_all_vecs_mut().into_par_iter()
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2_base(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            height_to_market_cap,
            exit,
        )
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.supply).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.outputs).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.activity).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.realized).collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.unrealized).collect::<Vec<_>>(),
            exit,
        )?;
        self.cost_basis.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.cost_basis).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
