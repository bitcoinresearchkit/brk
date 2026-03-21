use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Cents, Dollars, Height, Indexes, Version,
};
use vecdb::AnyStoredVec;
use vecdb::{Exit, ReadOnlyClone, ReadableVec, Rw, StorageMode};

use crate::{
    blocks,
    distribution::metrics::{
        ActivityFull, AdjustedSopr, CohortMetricsBase, CostBasis, ImportConfig, OutputsBase,
        RealizedFull, RelativeForAll, SupplyCore, UnrealizedFull,
    },
    prices,
};

/// All-cohort metrics: extended realized + adjusted (as composable add-on),
/// extended cost basis, relative for-all (no rel_to_all).
/// Used by: the "all" cohort.
#[derive(Traversable)]
pub struct AllCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyCore<M>>,
    pub outputs: Box<OutputsBase<M>>,
    pub activity: Box<ActivityFull<M>>,
    pub realized: Box<RealizedFull<M>>,
    pub cost_basis: Box<CostBasis<M>>,
    pub unrealized: Box<UnrealizedFull<M>>,
    #[traversable(wrap = "realized/sopr", rename = "adjusted")]
    pub asopr: Box<AdjustedSopr<M>>,
    #[traversable(flatten)]
    pub relative: Box<RelativeForAll<M>>,
}

impl CohortMetricsBase for AllCohortMetrics {
    type ActivityVecs = ActivityFull;
    type RealizedVecs = RealizedFull;
    type UnrealizedVecs = UnrealizedFull;

    impl_cohort_accessors!();

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;
        self.cost_basis.validate_computed_versions(base_version)?;
        Ok(())
    }

    fn min_stateful_len(&self) -> usize {
        // Only check per-block pushed vecs, not aggregated ones (supply, outputs,
        // activity, realized core, unrealized core are summed from age_range).
        self.realized
            .min_stateful_len()
            .min(self.unrealized.min_stateful_len())
            .min(self.cost_basis.min_stateful_len())
    }

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.cost_basis.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }
}

impl AllCohortMetrics {
    /// Import the "all" cohort metrics with a pre-imported supply.
    ///
    /// Supply is imported first (before other cohorts) so it can be used as `all_supply`
    /// reference for relative metric lazy vecs in other cohorts.
    pub(crate) fn forced_import_with_supply(
        cfg: &ImportConfig,
        supply: SupplyCore,
    ) -> Result<Self> {
        let unrealized = UnrealizedFull::forced_import(cfg)?;
        let realized = RealizedFull::forced_import(cfg)?;
        let asopr = AdjustedSopr::forced_import(cfg)?;

        let relative = RelativeForAll::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            activity: Box::new(ActivityFull::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasis::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            asopr: Box::new(asopr),
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
        under_1h_value_created: &impl ReadableVec<Height, Cents>,
        under_1h_value_destroyed: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            height_to_market_cap,
            &self.activity.transfer_volume,
            exit,
        )?;

        self.unrealized.compute(
            starting_indexes.height,
            &prices.spot.cents.height,
            &self.realized.price.cents.height,
            exit,
        )?;

        self.asopr.compute_rest_part2(
            starting_indexes,
            &self.activity.transfer_volume.block.cents,
            &self.realized.core.sopr.value_destroyed.block,
            under_1h_value_created,
            under_1h_value_destroyed,
            exit,
        )?;

        let all_utxo_count = self.outputs.unspent_count.height.read_only_clone();
        self.outputs.compute_part2(
            starting_indexes.height,
            &all_utxo_count,
            exit,
        )?;

        self.cost_basis.compute_prices(
            starting_indexes,
            &prices.spot.cents.height,
            &self.unrealized.invested_capital.in_profit.cents.height,
            &self.unrealized.invested_capital.in_loss.cents.height,
            &self.supply.in_profit.sats.height,
            &self.supply.in_loss.sats.height,
            &self.unrealized.investor_cap_in_profit_raw,
            &self.unrealized.investor_cap_in_loss_raw,
            exit,
        )?;

        self.unrealized.compute_sentiment(
            starting_indexes,
            &prices.spot.cents.height,
            exit,
        )?;

        self.relative.compute(
            starting_indexes.height,
            &self.supply,
            &self.unrealized,
            height_to_market_cap,
            exit,
        )?;

        Ok(())
    }
}
