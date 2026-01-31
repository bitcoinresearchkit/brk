use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CentsSats, CentsSquaredSats, CentsUnsigned, DateIndex, Dollars, Height, Sats};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, GenericStoredVec, ImportableVec, Negate,
    TypedVecIterator, Version,
};

use crate::{
    ComputeIndexes,
    distribution::state::UnrealizedState,
    indexes,
    internal::{
        ComputedFromHeightAndDateLast, ComputedFromHeightLast, DollarsMinus, DollarsPlus,
        LazyBinaryFromHeightLast, LazyFromHeightLast, ValueFromHeightAndDateLast,
    },
    price,
};

use super::ImportConfig;

/// Unrealized profit/loss metrics.
#[derive(Clone, Traversable)]
pub struct UnrealizedMetrics {
    // === Supply in Profit/Loss ===
    pub supply_in_profit: ValueFromHeightAndDateLast,
    pub supply_in_loss: ValueFromHeightAndDateLast,

    // === Unrealized Profit/Loss ===
    pub unrealized_profit: ComputedFromHeightAndDateLast<Dollars>,
    pub unrealized_loss: ComputedFromHeightAndDateLast<Dollars>,

    // === Invested Capital in Profit/Loss ===
    pub invested_capital_in_profit: ComputedFromHeightAndDateLast<Dollars>,
    pub invested_capital_in_loss: ComputedFromHeightAndDateLast<Dollars>,

    // === Raw values for precise aggregation (used to compute pain/greed indices) ===
    /// Σ(price × sats) for UTXOs in profit (raw u128, no indexes)
    pub invested_capital_in_profit_raw: BytesVec<Height, CentsSats>,
    /// Σ(price × sats) for UTXOs in loss (raw u128, no indexes)
    pub invested_capital_in_loss_raw: BytesVec<Height, CentsSats>,
    /// Σ(price² × sats) for UTXOs in profit (raw u128, no indexes)
    pub investor_cap_in_profit_raw: BytesVec<Height, CentsSquaredSats>,
    /// Σ(price² × sats) for UTXOs in loss (raw u128, no indexes)
    pub investor_cap_in_loss_raw: BytesVec<Height, CentsSquaredSats>,

    // === Pain/Greed Indices (computed in compute_rest from raw values + spot price) ===
    /// investor_price_of_losers - spot (average distance underwater, weighted by $)
    pub pain_index: ComputedFromHeightLast<Dollars>,
    /// spot - investor_price_of_winners (average distance in profit, weighted by $)
    pub greed_index: ComputedFromHeightLast<Dollars>,
    /// greed_index - pain_index (positive = greedy market, negative = painful market)
    pub net_sentiment: ComputedFromHeightLast<Dollars>,

    // === Negated ===
    pub neg_unrealized_loss: LazyFromHeightLast<Dollars>,

    // === Net and Total ===
    pub net_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,
    pub total_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,

    // === ATH Regret ===
    /// Unrealized ATH regret: (ATH - spot) × supply_in_profit + ATH × supply_in_loss - invested_capital_in_loss
    /// "How much more I'd have if I sold at ATH instead of now" (refined formula accounting for cost basis)
    pub ath_regret: ComputedFromHeightLast<Dollars>,
}

impl UnrealizedMetrics {
    /// Import unrealized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let compute_dollars = cfg.compute_dollars();

        // === Supply in Profit/Loss ===
        let supply_in_profit = ValueFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_profit"),
            cfg.version,
            compute_dollars,
            cfg.indexes,
            cfg.price,
        )?;
        let supply_in_loss = ValueFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_loss"),
            cfg.version,
            compute_dollars,
            cfg.indexes,
            cfg.price,
        )?;

        // === Unrealized Profit/Loss ===
        let unrealized_profit = ComputedFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let unrealized_loss = ComputedFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        // === Invested Capital in Profit/Loss ===
        let invested_capital_in_profit = ComputedFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("invested_capital_in_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let invested_capital_in_loss = ComputedFromHeightAndDateLast::forced_import(
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
            cfg.version,
            cfg.indexes,
        )?;

        // === Negated ===
        let neg_unrealized_loss = LazyFromHeightLast::from_computed_height_date::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            &unrealized_loss,
        );

        // === Net and Total ===
        let net_unrealized_pnl =
            LazyBinaryFromHeightLast::from_computed_height_date_last::<DollarsMinus>(
                &cfg.name("net_unrealized_pnl"),
                cfg.version,
                &unrealized_profit,
                &unrealized_loss,
            );
        let total_unrealized_pnl =
            LazyBinaryFromHeightLast::from_computed_height_date_last::<DollarsPlus>(
                &cfg.name("total_unrealized_pnl"),
                cfg.version,
                &unrealized_profit,
                &unrealized_loss,
            );

        // === ATH Regret ===
        // v2: Changed to use HIGH prices consistently for ATH instead of mixing HIGH/CLOSE
        // v3: Changed to ComputedFromHeightLast to derive dateindex from height (avoids precision loss)
        let v3 = Version::new(3);
        let ath_regret = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_ath_regret"),
            cfg.version + v3,
            cfg.indexes,
        )?;

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
            ath_regret,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .height
            .len()
            .min(self.supply_in_loss.height.len())
            .min(self.unrealized_profit.height.len())
            .min(self.unrealized_loss.height.len())
            .min(self.invested_capital_in_profit.height.len())
            .min(self.invested_capital_in_loss.height.len())
            .min(self.invested_capital_in_profit_raw.len())
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len())
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        self.supply_in_profit
            .indexes
            .sats_dateindex
            .len()
            .min(self.supply_in_loss.indexes.sats_dateindex.len())
            .min(self.unrealized_profit.dateindex.len())
            .min(self.unrealized_loss.dateindex.len())
            .min(self.invested_capital_in_profit.dateindex.len())
            .min(self.invested_capital_in_loss.dateindex.len())
    }

    /// Push unrealized state values to height-indexed vectors.
    pub fn truncate_push(
        &mut self,
        height: Height,
        dateindex: Option<DateIndex>,
        height_state: &UnrealizedState,
        date_state: Option<&UnrealizedState>,
    ) -> Result<()> {
        self.supply_in_profit
            .height
            .truncate_push(height, height_state.supply_in_profit)?;
        self.supply_in_loss
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

        if let (Some(dateindex), Some(date_state)) = (dateindex, date_state) {
            self.supply_in_profit
                .indexes
                .sats_dateindex
                .truncate_push(dateindex, date_state.supply_in_profit)?;
            self.supply_in_loss
                .indexes
                .sats_dateindex
                .truncate_push(dateindex, date_state.supply_in_loss)?;
            self.unrealized_profit
                .dateindex
                .truncate_push(dateindex, date_state.unrealized_profit.to_dollars())?;
            self.unrealized_loss
                .dateindex
                .truncate_push(dateindex, date_state.unrealized_loss.to_dollars())?;
            self.invested_capital_in_profit.dateindex.truncate_push(
                dateindex,
                date_state.invested_capital_in_profit.to_dollars(),
            )?;
            self.invested_capital_in_loss
                .dateindex
                .truncate_push(dateindex, date_state.invested_capital_in_loss.to_dollars())?;
        }

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_loss.height as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_profit.height as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_loss.height as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_loss_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.indexes.sats_dateindex as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.indexes.sats_dateindex as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.rest.dateindex as &mut dyn AnyStoredVec,
            &mut self.unrealized_loss.rest.dateindex as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_profit.rest.dateindex as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_loss.rest.dateindex as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.supply_in_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_loss.height)
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
        // Create iterators for each source vec
        let mut iters: Vec<_> = others
            .iter()
            .filter_map(|o| {
                Some((
                    o.invested_capital_in_profit_raw.iter().ok()?,
                    o.invested_capital_in_loss_raw.iter().ok()?,
                    o.investor_cap_in_profit_raw.iter().ok()?,
                    o.investor_cap_in_loss_raw.iter().ok()?,
                ))
            })
            .collect();

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

            for (ip_iter, il_iter, cap_p_iter, cap_l_iter) in &mut iters {
                sum_invested_profit += ip_iter.get_unwrap(height);
                sum_invested_loss += il_iter.get_unwrap(height);
                sum_investor_profit += cap_p_iter.get_unwrap(height);
                sum_investor_loss += cap_l_iter.get_unwrap(height);
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

        self.supply_in_profit
            .indexes
            .sats_dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.supply_in_profit.indexes.sats_dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.supply_in_loss
            .indexes
            .sats_dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.supply_in_loss.indexes.sats_dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.unrealized_profit.dateindex.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.unrealized_profit.dateindex)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_loss.dateindex.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.unrealized_loss.dateindex)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.invested_capital_in_profit
            .dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.invested_capital_in_profit.dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.invested_capital_in_loss
            .dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.invested_capital_in_loss.dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        Ok(())
    }

    /// Compute derived metrics from stored values + price.
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit
            .compute_dollars_from_price(price, starting_indexes, exit)?;

        self.supply_in_loss
            .compute_dollars_from_price(price, starting_indexes, exit)?;

        // Compute pain/greed/net from raw values + spot price
        let Some(price) = price else {
            return Ok(());
        };

        // Pain index: investor_price_of_losers - spot
        self.pain_index
            .compute_all(indexes, starting_indexes, exit, |vec| {
                Ok(vec.compute_transform3(
                    starting_indexes.height,
                    &self.investor_cap_in_loss_raw,
                    &self.invested_capital_in_loss_raw,
                    &price.cents.split.height.close,
                    |(h, investor_cap, invested_cap, spot, ..)| {
                        if invested_cap.inner() == 0 {
                            return (h, Dollars::ZERO);
                        }
                        let investor_price_losers = investor_cap.inner() / invested_cap.inner();
                        let spot_u128 = (*spot).as_u128();
                        (
                            h,
                            CentsUnsigned::new((investor_price_losers - spot_u128) as u64)
                                .to_dollars(),
                        )
                    },
                    exit,
                )?)
            })?;

        // Greed index: spot - investor_price_of_winners
        self.greed_index
            .compute_all(indexes, starting_indexes, exit, |vec| {
                Ok(vec.compute_transform3(
                    starting_indexes.height,
                    &self.investor_cap_in_profit_raw,
                    &self.invested_capital_in_profit_raw,
                    &price.cents.split.height.close,
                    |(h, investor_cap, invested_cap, spot, ..)| {
                        if invested_cap.inner() == 0 {
                            return (h, Dollars::ZERO);
                        }
                        let investor_price_winners = investor_cap.inner() / invested_cap.inner();
                        let spot_u128 = (*spot).as_u128();
                        (
                            h,
                            CentsUnsigned::new((spot_u128 - investor_price_winners) as u64)
                                .to_dollars(),
                        )
                    },
                    exit,
                )?)
            })?;

        // Net sentiment: greed - pain
        self.net_sentiment
            .compute_all(indexes, starting_indexes, exit, |vec| {
                Ok(vec.compute_subtract(
                    starting_indexes.height,
                    &self.greed_index.height,
                    &self.pain_index.height,
                    exit,
                )?)
            })?;

        // ATH regret: (ATH - spot) × supply_in_profit + ATH × supply_in_loss - invested_capital_in_loss
        // This is the refined formula that accounts for cost basis:
        // - For UTXOs in profit: regret = ATH - spot (they could have sold at ATH instead of now)
        // - For UTXOs in loss: regret = ATH - cost_basis (they could have sold at ATH instead of holding)
        // ath = running max of high prices

        // Height computation
        {
            // Pre-compute ATH as running max of high prices
            let height_ath: Vec<CentsUnsigned> = {
                let mut ath = CentsUnsigned::ZERO;
                price
                    .cents
                    .split
                    .height
                    .high
                    .into_iter()
                    .map(|high| {
                        if *high > ath {
                            ath = *high;
                        }
                        ath
                    })
                    .collect()
            };

            self.ath_regret.height.compute_transform4(
                starting_indexes.height,
                &price.cents.split.height.close,
                &self.supply_in_profit.height,
                &self.supply_in_loss.height,
                &self.invested_capital_in_loss_raw,
                |(h, spot, supply_profit, supply_loss, invested_loss_raw, ..)| {
                    let ath = height_ath[usize::from(h)];
                    // (ATH - spot) × supply_in_profit + ATH × supply_in_loss - invested_capital_in_loss
                    let ath_u128 = ath.as_u128();
                    let spot_u128 = spot.as_u128();
                    let profit_regret = (ath_u128 - spot_u128) * supply_profit.as_u128();
                    // invested_loss_raw is CentsSats (already in cents*sats scale)
                    let loss_regret = ath_u128 * supply_loss.as_u128() - invested_loss_raw.inner();
                    let regret_raw = profit_regret + loss_regret;
                    let regret_cents = CentsUnsigned::new((regret_raw / Sats::ONE_BTC_U128) as u64);
                    (h, regret_cents.to_dollars())
                },
                exit,
            )?;
        }

        // DateIndex computation: derive from height values using last-value aggregation
        self.ath_regret
            .compute_rest(indexes, starting_indexes, exit)?;

        Ok(())
    }
}
