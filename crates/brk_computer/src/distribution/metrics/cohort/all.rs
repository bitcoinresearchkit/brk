use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Indexes, StoredF32, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::internal::ComputedFromHeight;

use crate::distribution::metrics::{
    ActivityFull, CostBasisWithExtended, ImportConfig, OutputsMetrics, RealizedAdjusted,
    RealizedFull, RelativeForAll, SupplyMetrics, UnrealizedFull,
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
    pub activity: Box<ActivityFull<M>>,
    pub realized: Box<RealizedFull<M>>,
    pub cost_basis: Box<CostBasisWithExtended<M>>,
    pub unrealized: Box<UnrealizedFull<M>>,
    pub adjusted: Box<RealizedAdjusted<M>>,
    pub relative: Box<RelativeForAll<M>>,
    pub dormancy: ComputedFromHeight<StoredF32, M>,
    pub velocity: ComputedFromHeight<StoredF32, M>,
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
        let unrealized = UnrealizedFull::forced_import(cfg)?;
        let realized = RealizedFull::forced_import(cfg)?;
        let adjusted = RealizedAdjusted::forced_import(cfg)?;

        let relative = RelativeForAll::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityFull::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisWithExtended::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            adjusted: Box::new(adjusted),
            relative: Box::new(relative),
            dormancy: cfg.import("dormancy", Version::ONE)?,
            velocity: cfg.import("velocity", Version::ONE)?,
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
            &self.realized,
            &self.supply.total.sats.height,
            height_to_market_cap,
            exit,
        )?;

        self.dormancy.height.compute_transform2(
            starting_indexes.height,
            &self.activity.coindays_destroyed.height,
            &self.activity.sent.base.sats.height,
            |(i, cdd, sent_sats, ..)| {
                let sent_btc = f64::from(Bitcoin::from(sent_sats));
                if sent_btc == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    (i, StoredF32::from((f64::from(cdd) / sent_btc) as f32))
                }
            },
            exit,
        )?;

        self.velocity.height.compute_transform2(
            starting_indexes.height,
            &self.activity.sent.base.sats.height,
            &self.supply.total.sats.height,
            |(i, sent_sats, supply_sats, ..)| {
                let supply = supply_sats.as_u128() as f64;
                if supply == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    (i, StoredF32::from((sent_sats.as_u128() as f64 / supply) as f32))
                }
            },
            exit,
        )?;

        Ok(())
    }
}
