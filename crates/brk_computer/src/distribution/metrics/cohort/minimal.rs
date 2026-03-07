use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{blocks, prices};

use crate::internal::ValueFromHeight;

use crate::distribution::{
    metrics::{ActivityBase, ImportConfig, OutputsMetrics, RealizedMinimal, SupplyMetrics},
    state::UnrealizedState,
};

/// Minimal unrealized metrics: supply in profit/loss only.
#[derive(Traversable)]
pub struct MinimalUnrealized<M: StorageMode = Rw> {
    pub supply_in_profit: ValueFromHeight<M>,
    pub supply_in_loss: ValueFromHeight<M>,
}

/// MinimalCohortMetrics: supply, outputs, sent+ema, realized cap/price/mvrv/profit/loss,
/// supply in profit/loss.
///
/// Used for type_, amount, and address cohorts.
/// Does NOT implement CohortMetricsBase — standalone, not aggregatable via trait.
#[derive(Traversable)]
pub struct MinimalCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityBase<M>>,
    pub realized: Box<RealizedMinimal<M>>,
    pub unrealized: Box<MinimalUnrealized<M>>,
}

impl MinimalUnrealized {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            supply_in_profit: cfg.import("supply_in_profit", Version::ZERO)?,
            supply_in_loss: cfg.import("supply_in_loss", Version::ZERO)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &UnrealizedState,
    ) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .truncate_push(height, state.supply_in_profit)?;
        self.supply_in_loss
            .sats
            .height
            .truncate_push(height, state.supply_in_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.cents.height as &mut dyn AnyStoredVec,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; supply_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; supply_in_loss.base.sats.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.compute(prices, max_from, exit)?;
        self.supply_in_loss.compute(prices, max_from, exit)?;
        Ok(())
    }
}

impl MinimalCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyMetrics::forced_import(cfg)?),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityBase::forced_import(cfg)?),
            realized: Box::new(RealizedMinimal::forced_import(cfg)?),
            unrealized: Box::new(MinimalUnrealized::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_height_len())
            .min(self.unrealized.min_stateful_height_len())
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
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

    /// Aggregate Minimal-tier metrics from other MinimalCohortMetrics sources.
    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&MinimalCohortMetrics],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.supply.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.outputs.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.activity.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized.compute_from_stateful(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.realized.as_ref())
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized.compute_from_sources(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.unrealized.as_ref())
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute(prices, starting_indexes.height, exit)?;
        self.supply
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs
            .compute_rest(blocks, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.realized
            .compute_rest_part1(starting_indexes, exit)?;
        self.unrealized
            .compute_rest(prices, starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            exit,
        )?;

        Ok(())
    }
}
