use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Version};
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::distribution::metrics::unrealized::UnrealizedMinimal;
use crate::{blocks, prices};

use crate::distribution::metrics::{
    ActivityCore, ImportConfig, OutputsMetrics, RealizedMinimal, SupplyMetrics,
};

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
    pub activity: Box<ActivityCore<M>>,
    pub realized: Box<RealizedMinimal<M>>,
    pub unrealized: Box<UnrealizedMinimal<M>>,
}

impl MinimalCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyMetrics::forced_import(cfg)?),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityCore::forced_import(cfg)?),
            realized: Box::new(RealizedMinimal::forced_import(cfg)?),
            unrealized: Box::new(UnrealizedMinimal::forced_import(cfg)?),
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
            &others
                .iter()
                .map(|v| v.outputs.as_ref())
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.activity.as_ref())
                .collect::<Vec<_>>(),
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
        self.supply.compute(prices, starting_indexes.height, exit)?;
        self.supply
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs.compute_rest(blocks, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.realized
            .compute_rest_part1(blocks, starting_indexes, exit)?;
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
