use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Indexes, Sats, StoredF32, Version};
use vecdb::AnyStoredVec;
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::internal::ComputedFromHeight;

use crate::distribution::metrics::{
    ActivityFull, CohortMetricsBase, CostBasisWithExtended, ImportConfig, OutputsMetrics,
    RealizedFull, RelativeWithExtended, SupplyMetrics, UnrealizedFull,
};

/// Cohort metrics with extended realized + extended cost basis (no adjusted).
/// Used by: lth, age_range cohorts.
#[derive(Traversable)]
pub struct ExtendedCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityFull<M>>,
    pub realized: Box<RealizedFull<M>>,
    pub cost_basis: Box<CostBasisWithExtended<M>>,
    pub unrealized: Box<UnrealizedFull<M>>,
    pub relative: Box<RelativeWithExtended<M>>,
    pub dormancy: ComputedFromHeight<StoredF32, M>,
    pub velocity: ComputedFromHeight<StoredF32, M>,
}

impl CohortMetricsBase for ExtendedCohortMetrics {
    type RealizedVecs = RealizedFull;
    type CostBasisVecs = CostBasisWithExtended;

    impl_cohort_accessors!();

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.cost_basis.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs.push(&mut self.dormancy.height);
        vecs.push(&mut self.velocity.height);
        vecs
    }
}

impl ExtendedCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let unrealized = UnrealizedFull::forced_import(cfg)?;
        let realized = RealizedFull::forced_import(cfg)?;

        let relative = RelativeWithExtended::forced_import(cfg)?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityFull::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisWithExtended::forced_import(cfg)?),
            unrealized: Box::new(unrealized),
            relative: Box::new(relative),
            dormancy: cfg.import("dormancy", Version::ONE)?,
            velocity: cfg.import("velocity", Version::ONE)?,
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
            &self.realized,
            &self.supply.total.sats.height,
            height_to_market_cap,
            all_supply_sats,
            &self.supply.total.usd.height,
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
