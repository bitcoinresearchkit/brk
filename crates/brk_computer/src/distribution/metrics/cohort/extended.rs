use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Indexes, Sats, StoredU64, Version};
use vecdb::AnyStoredVec;
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    blocks,
    distribution::metrics::{
        ActivityFull, CohortMetricsBase, CostBasis, ImportConfig, OutputsBase, RealizedFull,
        RelativeWithExtended, SupplyCore, UnrealizedFull,
    },
    prices,
};

/// Cohort metrics with extended realized + extended cost basis (no adjusted).
/// Used by: lth, age_range cohorts.
#[derive(Traversable)]
pub struct ExtendedCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyCore<M>>,
    pub outputs: Box<OutputsBase<M>>,
    pub activity: Box<ActivityFull<M>>,
    pub realized: Box<RealizedFull<M>>,
    pub cost_basis: Box<CostBasis<M>>,
    pub unrealized: Box<UnrealizedFull<M>>,
    #[traversable(flatten)]
    pub relative: Box<RelativeWithExtended<M>>,
}

impl CohortMetricsBase for ExtendedCohortMetrics {
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

impl ExtendedCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = SupplyCore::forced_import(cfg)?;
        let unrealized = UnrealizedFull::forced_import(cfg)?;
        let realized = RealizedFull::forced_import(cfg)?;

        let relative = RelativeWithExtended::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            activity: Box::new(ActivityFull::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasis::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
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
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        all_utxo_count: &impl ReadableVec<Height, StoredU64>,
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

        self.cost_basis.compute_prices(
            starting_indexes,
            &prices.spot.cents.height,
            &self.unrealized.invested_capital.in_profit.cents.height,
            &self.unrealized.invested_capital.in_loss.cents.height,
            &self.supply.in_profit.sats.height,
            &self.supply.in_loss.sats.height,
            &self.unrealized.capitalized_cap_in_profit_raw,
            &self.unrealized.capitalized_cap_in_loss_raw,
            exit,
        )?;

        self.unrealized
            .compute_sentiment(starting_indexes, &prices.spot.cents.height, exit)?;

        self.relative.compute(
            starting_indexes.height,
            &self.supply,
            &self.unrealized,
            height_to_market_cap,
            all_supply_sats,
            &self.supply.total.usd.height,
            exit,
        )?;

        self.outputs
            .compute_part2(starting_indexes.height, all_utxo_count, exit)?;

        Ok(())
    }
}
