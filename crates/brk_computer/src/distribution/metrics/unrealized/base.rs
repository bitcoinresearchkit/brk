use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSats, CentsSigned, CentsSquaredSats, Height, Indexes, Version};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode,
    WritableVec,
};

use crate::{
    distribution::state::UnrealizedState,
    internal::{
        CentsSubtractToCentsSigned, FiatFromHeight, LazyFromHeight, NegCentsUnsignedToDollars,
        ValueFromHeight,
    },
    prices,
};

use brk_types::Dollars;

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct UnrealizedBase<M: StorageMode = Rw> {
    pub supply_in_profit: ValueFromHeight<M>,
    pub supply_in_loss: ValueFromHeight<M>,

    pub unrealized_profit: FiatFromHeight<Cents, M>,
    pub unrealized_loss: FiatFromHeight<Cents, M>,

    pub invested_capital_in_profit: FiatFromHeight<Cents, M>,
    pub invested_capital_in_loss: FiatFromHeight<Cents, M>,

    pub invested_capital_in_profit_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub invested_capital_in_loss_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub pain_index: FiatFromHeight<Cents, M>,
    pub greed_index: FiatFromHeight<Cents, M>,
    pub net_sentiment: FiatFromHeight<CentsSigned, M>,

    pub neg_unrealized_loss: LazyFromHeight<Dollars, Cents>,

    pub net_unrealized_pnl: FiatFromHeight<CentsSigned, M>,
    pub gross_pnl: FiatFromHeight<Cents, M>,
}

impl UnrealizedBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let supply_in_profit = cfg.import_value("supply_in_profit", v0)?;
        let supply_in_loss = cfg.import_value("supply_in_loss", v0)?;

        let unrealized_profit = cfg.import_fiat("unrealized_profit", v0)?;
        let unrealized_loss = cfg.import_fiat("unrealized_loss", v0)?;

        let invested_capital_in_profit = cfg.import_fiat("invested_capital_in_profit", v0)?;
        let invested_capital_in_loss = cfg.import_fiat("invested_capital_in_loss", v0)?;

        let invested_capital_in_profit_raw =
            cfg.import_bytes("invested_capital_in_profit_raw", v0)?;
        let invested_capital_in_loss_raw = cfg.import_bytes("invested_capital_in_loss_raw", v0)?;
        let investor_cap_in_profit_raw = cfg.import_bytes("investor_cap_in_profit_raw", v0)?;
        let investor_cap_in_loss_raw = cfg.import_bytes("investor_cap_in_loss_raw", v0)?;

        let pain_index = cfg.import_fiat("pain_index", v0)?;
        let greed_index = cfg.import_fiat("greed_index", v0)?;
        let net_sentiment = cfg.import_fiat("net_sentiment", Version::ONE)?;

        let neg_unrealized_loss = LazyFromHeight::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            unrealized_loss.cents.height.read_only_boxed_clone(),
            &unrealized_loss.cents,
        );

        let net_unrealized_pnl = cfg.import_fiat("net_unrealized_pnl", v0)?;
        let gross_pnl = cfg.import_fiat("unrealized_gross_pnl", v0)?;

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
            gross_pnl,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
            .min(self.unrealized_profit.cents.height.len())
            .min(self.unrealized_loss.cents.height.len())
            .min(self.invested_capital_in_profit.cents.height.len())
            .min(self.invested_capital_in_loss.cents.height.len())
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
            .cents
            .height
            .truncate_push(height, height_state.unrealized_profit)?;
        self.unrealized_loss
            .cents
            .height
            .truncate_push(height, height_state.unrealized_loss)?;
        self.invested_capital_in_profit
            .cents
            .height
            .truncate_push(height, height_state.invested_capital_in_profit)?;
        self.invested_capital_in_loss
            .cents
            .height
            .truncate_push(height, height_state.invested_capital_in_loss)?;

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
            &mut self.supply_in_profit.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.cents.height,
            &mut self.unrealized_loss.cents.height,
            &mut self.invested_capital_in_profit.cents.height,
            &mut self.invested_capital_in_loss.cents.height,
            &mut self.invested_capital_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.invested_capital_in_loss_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec,
            &mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! sum_others {
            ($($field:tt).+) => {
                self.$($field).+.compute_sum_of_others(
                    starting_indexes.height,
                    &others.iter().map(|v| &v.$($field).+).collect::<Vec<_>>(),
                    exit,
                )?
            };
        }

        sum_others!(supply_in_profit.sats.height);
        sum_others!(supply_in_loss.sats.height);
        sum_others!(unrealized_profit.cents.height);
        sum_others!(unrealized_loss.cents.height);
        sum_others!(invested_capital_in_profit.cents.height);
        sum_others!(invested_capital_in_loss.cents.height);

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
            .map(|o| {
                o.invested_capital_in_profit_raw
                    .collect_range_at(start, end)
            })
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Pain index: investor_price_of_losers - spot
        self.pain_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_loss_raw,
            &self.invested_capital_in_loss_raw,
            &prices.price.cents.height,
            |(h, investor_cap, invested_cap, spot, ..)| {
                if invested_cap.inner() == 0 {
                    return (h, Cents::ZERO);
                }
                let investor_price_losers = investor_cap.inner() / invested_cap.inner();
                let spot_u128 = spot.as_u128();
                (h, Cents::new((investor_price_losers - spot_u128) as u64))
            },
            exit,
        )?;

        // Greed index: spot - investor_price_of_winners
        self.greed_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_profit_raw,
            &self.invested_capital_in_profit_raw,
            &prices.price.cents.height,
            |(h, investor_cap, invested_cap, spot, ..)| {
                if invested_cap.inner() == 0 {
                    return (h, Cents::ZERO);
                }
                let investor_price_winners = investor_cap.inner() / invested_cap.inner();
                let spot_u128 = spot.as_u128();
                (h, Cents::new((spot_u128 - investor_price_winners) as u64))
            },
            exit,
        )?;

        self.net_unrealized_pnl
            .cents
            .height
            .compute_binary::<Cents, Cents, CentsSubtractToCentsSigned>(
                starting_indexes.height,
                &self.unrealized_profit.cents.height,
                &self.unrealized_loss.cents.height,
                exit,
            )?;
        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.unrealized_profit.cents.height,
            &self.unrealized_loss.cents.height,
            exit,
        )?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    pub(crate) fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.net_sentiment
            .cents
            .height
            .compute_binary::<Cents, Cents, CentsSubtractToCentsSigned>(
                starting_indexes.height,
                &self.greed_index.cents.height,
                &self.pain_index.cents.height,
                exit,
            )?;
        Ok(())
    }
}
