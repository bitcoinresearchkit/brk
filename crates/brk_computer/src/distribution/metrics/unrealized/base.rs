use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSats, CentsSquaredSats, Dollars, Height, Version};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, ImportableVec, Negate, ReadableCloneableVec, ReadableVec,
    Rw, StorageMode, WritableVec,
};

use crate::{
    ComputeIndexes,
    distribution::state::UnrealizedState,
    internal::{
        ComputedFromHeightLast, LazyFromHeightLast, ValueFromHeightLast,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

/// Base unrealized profit/loss metrics (always computed).
#[derive(Traversable)]
pub struct UnrealizedBase<M: StorageMode = Rw> {
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
    pub invested_capital_in_profit_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub invested_capital_in_loss_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // === Pain/Greed Indices ===
    pub pain_index: ComputedFromHeightLast<Dollars, M>,
    pub greed_index: ComputedFromHeightLast<Dollars, M>,
    pub net_sentiment: ComputedFromHeightLast<Dollars, M>,

    // === Negated ===
    pub neg_unrealized_loss: LazyFromHeightLast<Dollars>,

    // === Net and Total ===
    pub net_unrealized_pnl: ComputedFromHeightLast<Dollars, M>,
    pub total_unrealized_pnl: ComputedFromHeightLast<Dollars, M>,
}

impl UnrealizedBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply_in_profit = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let supply_in_loss = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_loss"),
            cfg.version,
            cfg.indexes,
        )?;

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
        let investor_cap_in_profit_raw = BytesVec::forced_import(
            cfg.db,
            &cfg.name("investor_cap_in_profit_raw"),
            cfg.version,
        )?;
        let investor_cap_in_loss_raw = BytesVec::forced_import(
            cfg.db,
            &cfg.name("investor_cap_in_loss_raw"),
            cfg.version,
        )?;

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
            cfg.version + Version::ONE,
            cfg.indexes,
        )?;

        let neg_unrealized_loss = LazyFromHeightLast::from_computed::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            unrealized_loss.height.read_only_boxed_clone(),
            &unrealized_loss,
        );

        let net_unrealized_pnl = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("net_unrealized_pnl"),
            cfg.version,
            cfg.indexes,
        )?;
        let total_unrealized_pnl = ComputedFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("total_unrealized_pnl"),
            cfg.version,
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
        })
    }

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

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        height_state: &UnrealizedState,
    ) -> Result<()> {
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

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.base.usd.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.usd.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.height,
            &mut self.unrealized_loss.height,
            &mut self.invested_capital_in_profit.height,
            &mut self.invested_capital_in_loss.height,
            &mut self.invested_capital_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_loss_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .compute_sum_of_others(
                starting_indexes.height,
                &others
                    .iter()
                    .map(|v| &v.supply_in_profit.sats.height)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.supply_in_loss
            .sats
            .height
            .compute_sum_of_others(
                starting_indexes.height,
                &others
                    .iter()
                    .map(|v| &v.supply_in_loss.sats.height)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.unrealized_profit
            .height
            .compute_sum_of_others(
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
        self.invested_capital_in_loss
            .height
            .compute_sum_of_others(
                starting_indexes.height,
                &others
                    .iter()
                    .map(|v| &v.invested_capital_in_loss.height)
                    .collect::<Vec<_>>(),
                exit,
            )?;

        // Raw values for aggregation
        let start = self
            .invested_capital_in_profit_raw
            .len()
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len());
        let end = others
            .iter()
            .map(|o| o.invested_capital_in_profit_raw.len())
            .min()
            .unwrap_or(0);

        // Pre-collect all cohort data to avoid per-element BytesVec reads in nested loop
        let invested_profit_ranges: Vec<Vec<CentsSats>> = others
            .iter()
            .map(|o| o.invested_capital_in_profit_raw.collect_range_at(start, end))
            .collect();
        let invested_loss_ranges: Vec<Vec<CentsSats>> = others
            .iter()
            .map(|o| o.invested_capital_in_loss_raw.collect_range_at(start, end))
            .collect();
        let investor_profit_ranges: Vec<Vec<CentsSquaredSats>> = others
            .iter()
            .map(|o| o.investor_cap_in_profit_raw.collect_range_at(start, end))
            .collect();
        let investor_loss_ranges: Vec<Vec<CentsSquaredSats>> = others
            .iter()
            .map(|o| o.investor_cap_in_loss_raw.collect_range_at(start, end))
            .collect();

        for i in start..end {
            let height = Height::from(i);
            let local_i = i - start;

            let mut sum_invested_profit = CentsSats::ZERO;
            let mut sum_invested_loss = CentsSats::ZERO;
            let mut sum_investor_profit = CentsSquaredSats::ZERO;
            let mut sum_investor_loss = CentsSquaredSats::ZERO;

            for idx in 0..others.len() {
                sum_invested_profit += invested_profit_ranges[idx][local_i];
                sum_invested_loss += invested_loss_ranges[idx][local_i];
                sum_investor_profit += investor_profit_ranges[idx][local_i];
                sum_investor_loss += investor_loss_ranges[idx][local_i];
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

        Ok(())
    }

    /// Compute derived metrics from stored values + price.
    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Pain index: investor_price_of_losers - spot
        self.pain_index.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_loss_raw,
            &self.invested_capital_in_loss_raw,
            &prices.price.cents,
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
            &prices.price.cents,
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

        self.net_unrealized_pnl.height.compute_subtract(
            starting_indexes.height,
            &self.unrealized_profit.height,
            &self.unrealized_loss.height,
            exit,
        )?;
        self.total_unrealized_pnl.height.compute_add(
            starting_indexes.height,
            &self.unrealized_profit.height,
            &self.unrealized_loss.height,
            exit,
        )?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
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
