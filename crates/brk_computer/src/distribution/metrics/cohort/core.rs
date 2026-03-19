use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::{
        ActivityCore, CohortMetricsBase, ImportConfig, OutputsBase, RealizedCore, RelativeToAll,
        SupplyCore, UnrealizedCore,
    },
    prices,
};

#[derive(Traversable)]
pub struct CoreCohortMetrics<M: StorageMode = Rw> {
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

impl CoreCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyCore::forced_import(cfg)?),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            activity: Box::new(ActivityCore::forced_import(cfg)?),
            realized: Box::new(RealizedCore::forced_import(cfg)?),
            unrealized: Box::new(UnrealizedCore::forced_import(cfg)?),
            relative: Box::new(RelativeToAll::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_len())
            .min(self.unrealized.min_stateful_len())
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;
        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }

    /// Aggregate Core-tier fields from CohortMetricsBase sources (e.g. age_range -> under_age/over_age).
    pub(crate) fn compute_from_base_sources<T: CohortMetricsBase>(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&T],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.supply()).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.outputs()).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.activity_core()).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.realized_core()).collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.unrealized_core()).collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute(prices, starting_indexes.height, exit)?;

        self.activity
            .compute_rest_part1(prices, starting_indexes, exit)?;

        self.realized
            .compute_rest_part1(starting_indexes, exit)?;

        self.unrealized.compute_rest(starting_indexes, exit)?;

        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
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

        Ok(())
    }
}
