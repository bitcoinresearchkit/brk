use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::distribution::metrics::{
    ActivityCore, CohortMetricsBase, ImportConfig, OutputsFull, RealizedCore,
    RelativeToAll, SupplyFull, UnrealizedBase,
};

/// Basic cohort metrics: no extensions, with relative (rel_to_all).
/// Used by: age_range cohorts.
#[derive(Traversable)]
pub struct BasicCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyFull<M>>,
    pub outputs: Box<OutputsFull<M>>,
    pub activity: Box<ActivityCore<M>>,
    pub realized: Box<RealizedCore<M>>,
    pub unrealized: Box<UnrealizedBase<M>>,
    pub relative: Box<RelativeToAll<M>>,
}

impl CohortMetricsBase for BasicCohortMetrics {
    type ActivityVecs = ActivityCore;
    type RealizedVecs = RealizedCore;
    type UnrealizedVecs = UnrealizedBase;

    impl_cohort_accessors!();

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }
}

impl BasicCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = SupplyFull::forced_import(cfg)?;
        let unrealized = UnrealizedBase::forced_import(cfg)?;
        let realized = RealizedCore::forced_import(cfg)?;

        let relative = RelativeToAll::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsFull::forced_import(cfg)?),
            activity: Box::new(ActivityCore::forced_import(cfg)?),
            realized: Box::new(realized),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
        })
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            exit,
        )?;

        self.relative.compute(
            starting_indexes.height,
            &self.unrealized.supply_in_profit.sats.height,
            &self.unrealized.supply_in_loss.sats.height,
            &self.supply.total.sats.height,
            all_supply_sats,
            exit,
        )?;

        Ok(())
    }

}
