use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, distribution::state::{CohortState, RealizedState}, prices};

use crate::distribution::metrics::{
    ActivityMetrics, CohortMetricsBase, CostBasisBase, CostBasisWithExtended, ImportConfig,
    OutputsMetrics, RealizedBase, RealizedWithExtended, RelativeWithExtended, SupplyMetrics,
    UnrealizedBase,
};

/// Cohort metrics with extended realized + extended cost basis (no adjusted).
/// Used by: lth, age_range cohorts.
#[derive(Traversable)]
pub struct ExtendedCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityMetrics<M>>,
    pub realized: Box<RealizedWithExtended<M>>,
    pub cost_basis: Box<CostBasisWithExtended<M>>,
    pub unrealized: Box<UnrealizedBase<M>>,
    pub relative: Box<RelativeWithExtended<M>>,
}

impl_cohort_metrics_base!(ExtendedCohortMetrics, extended_cost_basis);

impl ExtendedCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let unrealized = UnrealizedBase::forced_import(cfg)?;
        let realized = RealizedWithExtended::forced_import(cfg)?;

        let relative = RelativeWithExtended::forced_import(cfg)?;

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
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
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
            &self.realized.base,
            &self.supply.total.sats.height,
            height_to_market_cap,
            all_supply_sats,
            &self.supply.total.usd.height,
            exit,
        )?;

        Ok(())
    }
}
