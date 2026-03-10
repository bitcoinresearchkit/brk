use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Dollars, Height, Indexes, Sats, SatsSigned, StoredI64, StoredU64, Version,
};
use vecdb::AnyStoredVec;
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::internal::RollingDeltaExcept1m;

use crate::distribution::metrics::{
    ActivityFull, CohortMetricsBase, CostBasis, ImportConfig, OutputsFull,
    RealizedFull, RelativeWithExtended, SupplyFull, UnrealizedFull,
};

/// Cohort metrics with extended realized + extended cost basis (no adjusted).
/// Used by: lth, age_range cohorts.
#[derive(Traversable)]
pub struct ExtendedCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyFull<M>>,
    pub outputs: Box<OutputsFull<M>>,
    pub activity: Box<ActivityFull<M>>,
    pub realized: Box<RealizedFull<M>>,
    pub cost_basis: Box<CostBasis<M>>,
    pub unrealized: Box<UnrealizedFull<M>>,
    pub relative: Box<RelativeWithExtended<M>>,

    #[traversable(wrap = "supply", rename = "delta")]
    pub supply_delta_extended: RollingDeltaExcept1m<Sats, SatsSigned, M>,
    #[traversable(wrap = "outputs", rename = "utxo_count_delta")]
    pub utxo_count_delta_extended: RollingDeltaExcept1m<StoredU64, StoredI64, M>,
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

    fn min_stateful_height_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_height_len())
            .min(self.unrealized.min_stateful_height_len())
            .min(self.cost_basis.min_stateful_height_len())
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
        let supply = SupplyFull::forced_import(cfg)?;
        let unrealized = UnrealizedFull::forced_import(cfg)?;
        let realized = RealizedFull::forced_import(cfg)?;

        let relative = RelativeWithExtended::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsFull::forced_import(cfg)?),
            activity: Box::new(ActivityFull::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasis::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
            supply_delta_extended: cfg.import("supply_delta", Version::ONE)?,
            utxo_count_delta_extended: cfg.import("utxo_count_delta", Version::ONE)?,
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
            &self.supply.total.sats.height,
            height_to_market_cap,
            all_supply_sats,
            &self.supply.total.usd.height,
            exit,
        )?;

        let window_starts = blocks.lookback.window_starts();
        self.supply_delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.supply.total.sats.height,
            exit,
        )?;
        self.utxo_count_delta_extended.compute(
            starting_indexes.height,
            &window_starts,
            &self.outputs.utxo_count.height,
            exit,
        )?;

        self.activity.compute_rest_part2(
            starting_indexes,
            &self.supply.total.sats.height,
            exit,
        )?;

        Ok(())
    }
}
