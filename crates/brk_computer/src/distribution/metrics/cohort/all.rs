use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Indexes, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, distribution::state::{CohortState, RealizedState}, prices};

use crate::distribution::metrics::{
    ActivityMetrics, CohortMetricsBase, CostBasisBase, CostBasisWithExtended, ImportConfig,
    OutputsMetrics, RealizedAdjusted, RealizedBase, RealizedWithExtended, RelativeForAll,
    SupplyMetrics, UnrealizedBase,
};

/// All-cohort metrics: extended realized + adjusted (as composable add-on),
/// extended cost basis, relative for-all (no rel_to_all).
/// Used by: the "all" cohort.
#[derive(Traversable)]
pub struct AllCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityMetrics<M>>,
    pub realized: Box<RealizedWithExtended<M>>,
    pub cost_basis: Box<CostBasisWithExtended<M>>,
    pub unrealized: Box<UnrealizedBase<M>>,
    pub adjusted: Box<RealizedAdjusted<M>>,
    pub relative: Box<RelativeForAll<M>>,
}

impl_cohort_metrics_base!(AllCohortMetrics, extended_cost_basis);

impl AllCohortMetrics {
    /// Import the "all" cohort metrics with a pre-imported supply.
    ///
    /// Supply is imported first (before other cohorts) so it can be used as `all_supply`
    /// reference for relative metric lazy vecs in other cohorts.
    pub(crate) fn forced_import_with_supply(
        cfg: &ImportConfig,
        supply: SupplyMetrics,
    ) -> Result<Self> {
        let unrealized = UnrealizedBase::forced_import(cfg)?;
        let realized = RealizedWithExtended::forced_import(cfg)?;
        let adjusted = RealizedAdjusted::forced_import(cfg)?;

        let relative = RelativeForAll::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityMetrics::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisWithExtended::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            adjusted: Box::new(adjusted),
            relative: Box::new(relative),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        up_to_1h_value_created: &impl ReadableVec<Height, Cents>,
        up_to_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
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

        self.adjusted.compute_rest_part2(
            blocks,
            starting_indexes,
            &self.realized.value_created.height,
            &self.realized.value_destroyed.height,
            up_to_1h_value_created,
            up_to_1h_value_destroyed,
            exit,
        )?;

        self.relative.compute(
            starting_indexes.height,
            &self.unrealized,
            &self.realized.base,
            &self.supply.total.sats.height,
            height_to_market_cap,
            exit,
        )?;

        Ok(())
    }
}
