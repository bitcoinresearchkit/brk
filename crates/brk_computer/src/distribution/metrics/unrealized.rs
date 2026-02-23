use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSats, CentsSquaredSats, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, WritableVec, ImportableVec, ReadableCloneableVec,
    ReadableVec, Negate, Rw, StorageMode,
};

use crate::{
    ComputeIndexes,
    distribution::state::UnrealizedState,
    internal::{
        ComputedFromHeightLast, DollarsMinus, DollarsPlus,
        LazyBinaryFromHeightLast, LazyFromHeightLast, ValueFromHeightLast,
    },
    prices,
};

use super::ImportConfig;

/// Unrealized profit/loss metrics.
#[derive(Traversable)]
pub struct UnrealizedMetrics<M: StorageMode = Rw> {
    // === Supply in Profit/Loss ===
    pub supply_in_profit: ValueFromHeightLast<M>,
    pub supply_in_loss: ValueFromHeightLast<M>,

    // === Unrealized Profit/Loss ===
    pub unrealized_profit: ComputedFromHeightLast<Dollars, M>,
    pub unrealized_loss: ComputedFromHeightLast<Dollars, M>,

    // === Invested Capital in Profit/Loss ===
    pub invested_capital_in_profit: ComputedFromHeightLast<Dollars, M>,
    pub invested_capital_in_loss: ComputedFromHeightLast<Dollars, M>,

    // === Raw values for precise aggregation (used to compute pain/greed indices) ===
    /// Σ(price × sats) for UTXOs in profit (raw u128, no indexes)
    pub invested_capital_in_profit_raw: M::Stored<BytesVec<Height, CentsSats>>,
    /// Σ(price × sats) for UTXOs in loss (raw u128, no indexes)
    pub invested_capital_in_loss_raw: M::Stored<BytesVec<Height, CentsSats>>,
    /// Σ(price² × sats) for UTXOs in profit (raw u128, no indexes)
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    /// Σ(price² × sats) for UTXOs in loss (raw u128, no indexes)
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // === Pain/Greed Indices (computed in compute_rest from raw values + spot price) ===
    /// investor_price_of_losers - spot (average distance underwater, weighted by $)
    pub pain_index: ComputedFromHeightLast<Dollars, M>,
    /// spot - investor_price_of_winners (average distance in profit, weighted by $)
    pub greed_index: ComputedFromHeightLast<Dollars, M>,
    /// greed_index - pain_index (positive = greedy market, negative = painful market)
    pub net_sentiment: ComputedFromHeightLast<Dollars, M>,

    // === Negated ===
    pub neg_unrealized_loss: LazyFromHeightLast<Dollars>,

    // === Net and Total ===
    pub net_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,
    pub total_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,

    // === Peak Regret (age_range cohorts only) ===
    /// Unrealized peak regret: sum of (peak_price - reference_price) × supply
    /// where reference_price = max(spot, cost_basis) and peak = max price during holding period.
    /// Only computed for age_range cohorts, then aggregated for overlapping cohorts.
    pub peak_regret: Option<ComputedFromHeightLast<Dollars, M>>,
}

impl UnrealizedMetrics {
    /// Import unrealized metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        // === Supply in Profit/Loss ===
        let supply_in_profit = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_profit"),
            cfg.version,
            cfg.indexes,
            cfg.prices,
        )?;
        let supply_in_loss = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_loss"),
            cfg.version,
            cfg.indexes,
            cfg.prices,
        )?;

        // === Unrealized Profit/Loss ===
        let unrealized_profit = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let unrealized_loss = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        // === Invested Capital in Profit/Loss ===
        let invested_capital_in_profit = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("invested_capital_in_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let invested_capital_in_loss = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("invested_capital_in_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        // === Raw values for precise aggregation ===
        let invested_capital_in_profit_raw = BytesVec::forced_import(
            cfg.db,
            &cfg.name("invested_capital_in_profit_raw"),
            cfg.version,
        )?;
        let invested_capital_in_loss_raw = BytesVec::forced_import(
            cfg.db,
            &cfg.name("invested_capital_in_loss_raw"),
            cfg.version,
        )?;
        let investor_cap_in_profit_raw =
            BytesVec::forced_import(cfg.db, &cfg.name("investor_cap_in_profit_raw"), cfg.version)?;
        let investor_cap_in_loss_raw =
            BytesVec::forced_import(cfg.db, &cfg.name("investor_cap_in_loss_raw"), cfg.version)?;

        // === Pain/Greed Indices ===
        let pain_index = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("pain_index"),
            cfg.version,
            cfg.indexes,
        )?;
        let greed_index = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("greed_index"),
            cfg.version,
            cfg.indexes,
        )?;
        let net_sentiment = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("net_sentiment"),
            cfg.version + Version::ONE, // v1: weighted average for aggregate cohorts
            cfg.indexes,
        )?;

        // === Negated ===
        let neg_unrealized_loss = LazyFromHeightLast::from_computed::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            unrealized_loss.height.read_only_boxed_clone(),
            &unrealized_loss,
        );

        // === Net and Total ===
        let net_unrealized_pnl = LazyBinaryFromHeightLast::from_computed_last::<DollarsMinus>(
            &cfg.name("net_unrealized_pnl"),
            cfg.version,
            &unrealized_profit,
            &unrealized_loss,
        );
        let total_unrealized_pnl = LazyBinaryFromHeightLast::from_computed_last::<DollarsPlus>(
            &cfg.name("total_unrealized_pnl"),
            cfg.version,
            &unrealized_profit,
            &unrealized_loss,
        );

        // Peak regret: only for age-based UTXO cohorts
        let peak_regret = cfg
            .compute_peak_regret()
            .then(|| {
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("unrealized_peak_regret"),
                    cfg.version,
                    cfg.indexes,
                )
            })
            .transpose()?;

        Ok(Self {
            supply_in_profit,
            supply_in_loss,
            unrealized_profit,
            unrealized_loss,
            invested_capital_in_profit,
            invested_capital_in_loss,
            invested_capital_in_profit_raw,
            invested_capital_in_loss_raw,
            investor_cap_in_profit_raw,
            investor_cap_in_loss_raw,
            pain_index,
            greed_index,
            net_sentiment,
            neg_unrealized_loss,
            net_unrealized_pnl,
            total_unrealized_pnl,
            peak_regret,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
            .min(self.unrealized_profit.height.len())
            .min(self.unrealized_loss.height.len())
            .min(self.invested_capital_in_profit.height.len())
            .min(self.invested_capital_in_loss.height.len())
            .min(self.invested_capital_in_profit_raw.len())
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len())
    }

    /// Push unrealized state values to height-indexed vectors.
    pub(crate) fn truncate_push(&mut self, height: Height, height_state: &UnrealizedState) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .truncate_push(height, height_state.supply_in_profit)?;
        self.supply_in_loss
            .sats
            .height
            .truncate_push(height, height_state.supply_in_loss)?;
        self.unrealized_profit
            .height
            .truncate_push(height, height_state.unrealized_profit.to_dollars())?;
        self.unrealized_loss
            .height
            .truncate_push(height, height_state.unrealized_loss.to_dollars())?;
        self.invested_capital_in_profit
            .height
            .truncate_push(height, height_state.invested_capital_in_profit.to_dollars())?;
        self.invested_capital_in_loss
            .height
            .truncate_push(height, height_state.invested_capital_in_loss.to_dollars())?;

        // Raw values for aggregation
        self.invested_capital_in_profit_raw.truncate_push(
            height,
            CentsSats::new(height_state.invested_capital_in_profit_raw),
        )?;
        self.invested_capital_in_loss_raw.truncate_push(
            height,
            CentsSats::new(height_state.invested_capital_in_loss_raw),
        )?;
        self.investor_cap_in_profit_raw.truncate_push(
            height,
            CentsSquaredSats::new(height_state.investor_cap_in_profit_raw),
        )?;
        self.investor_cap_in_loss_raw.truncate_push(
            height,
            CentsSquaredSats::new(height_state.investor_cap_in_loss_raw),
        )?;

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.supply_in_profit.sats.height,
            &mut self.supply_in_loss.sats.height,
            &mut self.unrealized_profit.height,
            &mut self.unrealized_loss.height,
            &mut self.invested_capital_in_profit.height,
            &mut self.invested_capital_in_loss.height,
            &mut self.invested_capital_in_profit_raw,
            &mut self.invested_capital_in_loss_raw,
            &mut self.investor_cap_in_profit_raw,
            &mut self.investor_cap_in_loss_raw,
        ];
        if let Some(pr) = &mut self.peak_regret {
            vecs.push(&mut pr.height);
        }
        vecs.into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_profit.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.supply_in_loss.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_loss.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.unrealized_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.unrealized_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.invested_capital_in_profit
            .height
            .compute_sum_of_others(
                starting_indexes.height,
                &others
                    .iter()
                    .map(|v| &v.invested_capital_in_profit.height)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.invested_capital_in_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.invested_capital_in_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        // Raw values for aggregation - manually sum since BytesVec doesn't have compute_sum_of_others
        // Start from where the target vecs left off (handles fresh/reset vecs)
        let start = self
            .invested_capital_in_profit_raw
            .len()
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len());
        // End at the minimum length across all source vecs
        let end = others
            .iter()
            .map(|o| o.invested_capital_in_profit_raw.len())
            .min()
            .unwrap_or(0);

        for i in start..end {
            let height = Height::from(i);

            let mut sum_invested_profit = CentsSats::ZERO;
            let mut sum_invested_loss = CentsSats::ZERO;
            let mut sum_investor_profit = CentsSquaredSats::ZERO;
            let mut sum_investor_loss = CentsSquaredSats::ZERO;

            for o in others.iter() {
                sum_invested_profit += o.invested_capital_in_profit_raw.collect_one_at(i).unwrap();
                sum_invested_loss += o.invested_capital_in_loss_raw.collect_one_at(i).unwrap();
                sum_investor_profit += o.investor_cap_in_profit_raw.collect_one_at(i).unwrap();
                sum_investor_loss += o.investor_cap_in_loss_raw.collect_one_at(i).unwrap();
            }

            self.invested_capital_in_profit_raw
                .truncate_push(height, sum_invested_profit)?;
            self.invested_capital_in_loss_raw
                .truncate_push(height, sum_invested_loss)?;
            self.investor_cap_in_profit_raw
                .truncate_push(height, sum_investor_profit)?;
            self.investor_cap_in_loss_raw
                .truncate_push(height, sum_investor_loss)?;
        }

        // Peak regret aggregation (only if this cohort has peak_regret)
        if let Some(pr) = &mut self.peak_regret {
            let other_prs: Vec<_> = others
                .iter()
                .filter_map(|v| v.peak_regret.as_ref())
                .collect();
            if !other_prs.is_empty() {
                pr.height.compute_sum_of_others(
                    starting_indexes.height,
                    &other_prs.iter().map(|v| &v.height).collect::<Vec<_>>(),
                    exit,
                )?;
            }
        }

        Ok(())
    }

    /// Compute derived metrics from stored values + price.
    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Height-based types now have lazy day1, no compute_rest needed.

        // Pain index: investor_price_of_losers - spot
        self.pain_index.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_loss_raw,
            &self.invested_capital_in_loss_raw,
            &prices.cents.price,
            |(h, investor_cap, invested_cap, spot, ..)| {
                if invested_cap.inner() == 0 {
                    return (h, Dollars::ZERO);
                }
                let investor_price_losers = investor_cap.inner() / invested_cap.inner();
                let spot_u128 = spot.as_u128();
                (
                    h,
                    Cents::new((investor_price_losers - spot_u128) as u64).to_dollars(),
                )
            },
            exit,
        )?;

        // Greed index: spot - investor_price_of_winners
        self.greed_index.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_profit_raw,
            &self.invested_capital_in_profit_raw,
            &prices.cents.price,
            |(h, investor_cap, invested_cap, spot, ..)| {
                if invested_cap.inner() == 0 {
                    return (h, Dollars::ZERO);
                }
                let investor_price_winners = investor_cap.inner() / invested_cap.inner();
                let spot_u128 = spot.as_u128();
                (
                    h,
                    Cents::new((spot_u128 - investor_price_winners) as u64).to_dollars(),
                )
            },
            exit,
        )?;

        // Net sentiment height (greed - pain) computed separately for separate cohorts only
        // Aggregate cohorts compute it via weighted average in compute_from_stateful
        // Dateindex derivation for ALL cohorts happens in compute_net_sentiment_rest

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    /// Aggregate cohorts skip this - their height is computed via weighted average in compute_from_stateful.
    pub(crate) fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        Ok(self.net_sentiment.height.compute_subtract(
            starting_indexes.height,
            &self.greed_index.height,
            &self.pain_index.height,
            exit,
        )?)
    }
}
