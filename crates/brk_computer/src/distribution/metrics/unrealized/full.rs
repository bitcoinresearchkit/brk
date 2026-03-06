use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSats, CentsSigned, CentsSquaredSats, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, BytesVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::state::UnrealizedState,
    internal::{CentsSubtractToCentsSigned, FiatFromHeight},
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::UnrealizedBase;

/// Full unrealized metrics (Source/Extended tier).
///
/// Contains all Complete-tier fields (via Deref to UnrealizedBase) plus:
/// - Source-only: invested_capital, raw BytesVecs
/// - Extended-only: pain_index, greed_index, net_sentiment
#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: UnrealizedBase<M>,

    // --- Source-only fields ---
    pub invested_capital_in_profit: FiatFromHeight<Cents, M>,
    pub invested_capital_in_loss: FiatFromHeight<Cents, M>,

    pub invested_capital_in_profit_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub invested_capital_in_loss_raw: M::Stored<BytesVec<Height, CentsSats>>,
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    // --- Extended-only fields ---
    pub pain_index: FiatFromHeight<Cents, M>,
    pub greed_index: FiatFromHeight<Cents, M>,
    pub net_sentiment: FiatFromHeight<CentsSigned, M>,
}

impl UnrealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;

        let base = UnrealizedBase::forced_import(cfg)?;

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

        Ok(Self {
            base,
            invested_capital_in_profit,
            invested_capital_in_loss,
            invested_capital_in_profit_raw,
            invested_capital_in_loss_raw,
            investor_cap_in_profit_raw,
            investor_cap_in_loss_raw,
            pain_index,
            greed_index,
            net_sentiment,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.base
            .min_stateful_height_len()
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
        self.base.truncate_push(height, height_state)?;

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
        let mut vecs = self.base.collect_vecs_mut();
        vecs.push(&mut self.invested_capital_in_profit.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_loss.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_profit_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_loss_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        // Delegate Complete-tier aggregation
        let base_refs: Vec<&UnrealizedBase> =
            others.iter().map(|o| &o.base).collect();
        self.base
            .compute_from_stateful(starting_indexes, &base_refs, exit)?;

        // Source-only: invested_capital
        sum_others!(self, starting_indexes, others, exit; invested_capital_in_profit.cents.height);
        sum_others!(self, starting_indexes, others, exit; invested_capital_in_loss.cents.height);

        // Source-only: raw BytesVec aggregation
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
        // Complete-tier: net_unrealized_pnl
        self.base.compute_rest(starting_indexes, exit)?;

        // Extended-only: Pain index (investor_price_of_losers - spot)
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

        // Extended-only: Greed index (spot - investor_price_of_winners)
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
