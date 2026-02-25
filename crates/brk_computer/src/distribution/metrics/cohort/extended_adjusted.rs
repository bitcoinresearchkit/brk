use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, distribution::state::CohortState, prices};

use crate::distribution::metrics::{
    ActivityMetrics, CohortMetricsBase, CostBasisBase, CostBasisWithExtended, ImportConfig,
    OutputsMetrics, RealizedBase, RealizedWithExtendedAdjusted, RelativeWithExtended,
    SupplyMetrics, UnrealizedBase, UnrealizedWithPeakRegret,
};

/// Cohort metrics with extended + adjusted realized, extended cost basis, peak regret.
/// Used by: sth cohort.
#[derive(Traversable)]
pub struct ExtendedAdjustedCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityMetrics<M>>,
    pub realized: Box<RealizedWithExtendedAdjusted<M>>,
    pub cost_basis: Box<CostBasisWithExtended<M>>,
    pub unrealized: Box<UnrealizedWithPeakRegret<M>>,
    pub relative: Box<RelativeWithExtended>,
}

impl CohortMetricsBase for ExtendedAdjustedCohortMetrics {
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
        self.cost_basis.validate_computed_versions(base_version)?;
        Ok(())
    }
    fn compute_then_truncate_push_unrealized_states(
        &mut self, height: Height, height_price: Cents, state: &mut CohortState,
    ) -> Result<()> {
        state.apply_pending();
        self.cost_basis.truncate_push_minmax(height, state)?;
        let (height_unrealized_state, _) = state.compute_unrealized_states(height_price, None);
        self.unrealized.base.truncate_push(height, &height_unrealized_state)?;
        let spot = height_price.to_dollars();
        self.cost_basis.extended.truncate_push_percentiles(height, state, spot)?;
        Ok(())
    }
    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.outputs.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.cost_basis.base.collect_vecs_mut());
        vecs.extend(self.cost_basis.extended.collect_vecs_mut());
        vecs.extend(self.unrealized.base.collect_vecs_mut());
        vecs.extend(self.unrealized.peak_regret_ext.collect_vecs_mut());
        vecs
    }
}

impl ExtendedAdjustedCohortMetrics {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        all_supply: &SupplyMetrics,
        up_to_1h: &RealizedBase,
    ) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let unrealized = UnrealizedWithPeakRegret::forced_import(cfg)?;
        let realized = RealizedWithExtendedAdjusted::forced_import(cfg, up_to_1h)?;

        let relative = RelativeWithExtended::forced_import(
            cfg,
            &unrealized.base,
            &supply,
            all_supply,
            &realized.base,
            &unrealized.peak_regret_ext.peak_regret,
        );

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityMetrics::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisWithExtended::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
        })
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            height_to_market_cap,
            exit,
        )
    }

}
