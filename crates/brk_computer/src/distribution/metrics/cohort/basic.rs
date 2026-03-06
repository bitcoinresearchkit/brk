use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

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
    pub relative: Box<RelativeWithRelToAll<M>>,
}

impl_cohort_metrics_base!(BasicCohortMetrics, base_cost_basis);

impl BasicCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let unrealized = UnrealizedBase::forced_import(cfg)?;
        let realized = RealizedBase::forced_import(cfg)?;

        let relative = RelativeWithRelToAll::forced_import(cfg)?;

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

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2_base(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            height_to_market_cap,
            exit,
        )?;

        self.relative.compute(
            starting_indexes.height,
            &self.unrealized,
            &self.realized,
            &self.supply.total.sats.height,
            height_to_market_cap,
            all_supply_sats,
            exit,
        )?;

        Ok(())
    }

}
