use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, StoredU64};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::{
        ActivityCore, CohortMetricsBase, ImportConfig, OutputsBase, RealizedCore, RelativeToAll,
        SupplyCore, UnrealizedCore,
    },
    prices,
};

/// Basic cohort metrics: no extensions, with relative (rel_to_all).
/// Used by: age_range cohorts.
#[derive(Traversable)]
pub struct BasicCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyCore<M>>,
    pub outputs: Box<OutputsBase<M>>,
    pub activity: Box<ActivityCore<M>>,
    pub realized: Box<RealizedCore<M>>,
    pub unrealized: Box<UnrealizedCore<M>>,
    #[traversable(flatten)]
    pub relative: Box<RelativeToAll<M>>,
}

impl CohortMetricsBase for BasicCohortMetrics {
    type ActivityVecs = ActivityCore;
    type RealizedVecs = RealizedCore;
    type UnrealizedVecs = UnrealizedCore;

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
        let supply = SupplyCore::forced_import(cfg)?;
        let unrealized = UnrealizedCore::forced_import(cfg)?;
        let realized = RealizedCore::forced_import(cfg)?;

        let relative = RelativeToAll::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            activity: Box::new(ActivityCore::forced_import(cfg)?),
            realized: Box::new(realized),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
        })
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        all_utxo_count: &impl ReadableVec<Height, StoredU64>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            &self.activity.transfer_volume.sum._24h.cents.height,
            exit,
        )?;

        self.unrealized.compute(
            starting_indexes.height,
            &prices.spot.cents.height,
            &self.realized.price.cents.height,
            exit,
        )?;

        self.relative.compute(
            starting_indexes.height,
            &self.supply,
            all_supply_sats,
            exit,
        )?;

        self.outputs.compute_part2(starting_indexes.height, all_utxo_count, exit)?;

        Ok(())
    }

}
