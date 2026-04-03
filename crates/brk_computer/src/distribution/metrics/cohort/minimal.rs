use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, StoredU64};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::{
        ActivityMinimal, ImportConfig, OutputsBase, RealizedMinimal, SupplyBase, UnrealizedMinimal,
    },
    prices,
};

/// MinimalCohortMetrics: supply, outputs, realized cap/price/mvrv/profit/loss + value_created/destroyed.
///
/// Used for amount_range cohorts.
/// Does NOT implement CohortMetricsBase — standalone, not aggregatable via trait.
#[derive(Traversable)]
pub struct MinimalCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyBase<M>>,
    pub outputs: Box<OutputsBase<M>>,
    pub activity: Box<ActivityMinimal<M>>,
    pub realized: Box<RealizedMinimal<M>>,
    pub unrealized: Box<UnrealizedMinimal<M>>,
}

impl MinimalCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyBase::forced_import(cfg)?),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            activity: Box::new(ActivityMinimal::forced_import(cfg)?),
            realized: Box::new(RealizedMinimal::forced_import(cfg)?),
            unrealized: Box::new(UnrealizedMinimal::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_len())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
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
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute(prices, starting_indexes.height, exit)?;
        self.outputs.compute_rest(starting_indexes.height, exit)?;
        self.activity
            .compute_rest_part1(prices, starting_indexes, exit)?;
        self.realized.compute_rest_part1(starting_indexes, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        all_utxo_count: &impl ReadableVec<Height, StoredU64>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            exit,
        )?;

        self.unrealized.compute(
            starting_indexes.height,
            &prices.cached_spot_cents,
            &self.realized.price.cents.height,
            exit,
        )?;

        self.outputs
            .compute_part2(starting_indexes.height, all_utxo_count, exit)?;

        Ok(())
    }
}
