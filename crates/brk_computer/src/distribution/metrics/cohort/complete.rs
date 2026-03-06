use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Indexes, Sats, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{blocks, prices};

use crate::distribution::metrics::{
    ActivityMetrics, CohortMetricsBase, CostBasisBase, ImportConfig, OutputsMetrics,
    RealizedComplete, RelativeCompleteWithRelToAll, SupplyMetrics, UnrealizedComplete,
};

/// Complete cohort metrics (Tier C): ~216 stored vecs.
///
/// Used for epoch, class, min_age, max_age cohorts.
/// Everything in Core, plus cost basis, CDD, value created/destroyed,
/// sent in profit/loss, net PnL change, etc.
///
/// Does NOT include source-only fields (peak_regret, invested_capital,
/// raw BytesVecs) or extended-only fields (investor_price, sell_side_risk,
/// pain/greed/net_sentiment).
///
/// Does NOT implement CohortMetricsBase — standalone, not usable as Source.
#[derive(Traversable)]
pub struct CompleteCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityMetrics<M>>,
    pub realized: Box<RealizedComplete<M>>,
    pub cost_basis: Box<CostBasisBase<M>>,
    pub unrealized: Box<UnrealizedComplete<M>>,
    pub relative: Box<RelativeCompleteWithRelToAll<M>>,
}

impl CompleteCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyMetrics::forced_import(cfg)?),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityMetrics::forced_import(cfg)?),
            realized: Box::new(RealizedComplete::forced_import(cfg)?),
            cost_basis: Box::new(CostBasisBase::forced_import(cfg)?),
            unrealized: Box::new(UnrealizedComplete::forced_import(cfg)?),
            relative: Box::new(RelativeCompleteWithRelToAll::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_height_len())
            .min(self.unrealized.min_stateful_height_len())
            .min(self.cost_basis.min_stateful_height_len())
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;
        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.outputs.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.cost_basis.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }

    /// Aggregate Complete-tier metrics from Source cohort refs.
    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&dyn CohortMetricsBase],
        exit: &Exit,
    ) -> Result<()> {
        // Supply, outputs, activity: use their existing compute_from_stateful
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
            &others.iter().map(|v| v.activity()).collect::<Vec<_>>(),
            exit,
        )?;

        // Realized: aggregate only Complete-tier fields from Source's RealizedBase
        let realized_complete_refs: Vec<&RealizedComplete> = others
            .iter()
            .map(|v| &v.realized_base().complete)
            .collect();
        self.realized
            .compute_from_stateful(starting_indexes, &realized_complete_refs, exit)?;

        // Unrealized: aggregate only Complete-tier fields
        let unrealized_complete_refs: Vec<&UnrealizedComplete> = others
            .iter()
            .map(|v| &v.unrealized_base().complete)
            .collect();
        self.unrealized
            .compute_from_stateful(starting_indexes, &unrealized_complete_refs, exit)?;

        // Cost basis: use existing aggregation
        self.cost_basis.compute_from_stateful(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.cost_basis_base())
                .collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    /// First phase: compute index transforms.
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
            .sent
            .compute(prices, starting_indexes.height, exit)?;
        self.activity
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;

        self.realized
            .sent_in_profit
            .compute(prices, starting_indexes.height, exit)?;
        self.realized
            .sent_in_loss
            .compute(prices, starting_indexes.height, exit)?;
        self.realized
            .compute_rest_part1(starting_indexes, exit)?;

        self.unrealized.compute_rest(starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase: compute relative metrics and remaining.
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
            exit,
        )?;

        Ok(())
    }
}
