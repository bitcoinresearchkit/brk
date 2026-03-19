use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Height, Indexes, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::distribution::state::UnrealizedState;
use crate::internal::{CentsSubtractToCentsSigned, FiatPerBlock};
use crate::{distribution::metrics::ImportConfig, prices};

use super::UnrealizedBase;

#[derive(Traversable)]
pub struct UnrealizedSentiment<M: StorageMode = Rw> {
    pub pain_index: FiatPerBlock<Cents, M>,
    pub greed_index: FiatPerBlock<Cents, M>,
    pub net: FiatPerBlock<CentsSigned, M>,
}

#[derive(Traversable)]
pub struct UnrealizedInvestedCapital<M: StorageMode = Rw> {
    pub in_profit: FiatPerBlock<Cents, M>,
    pub in_loss: FiatPerBlock<Cents, M>,
}

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: UnrealizedBase<M>,

    pub gross_pnl: FiatPerBlock<Cents, M>,
    pub invested_capital: UnrealizedInvestedCapital<M>,

    pub sentiment: UnrealizedSentiment<M>,
}

impl UnrealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let inner = UnrealizedBase::forced_import(cfg)?;

        let gross_pnl = cfg.import("unrealized_gross_pnl", v0)?;

        let sentiment = UnrealizedSentiment {
            pain_index: cfg.import("pain_index", v0)?,
            greed_index: cfg.import("greed_index", v0)?,
            net: cfg.import("net_sentiment", Version::ONE)?,
        };

        let invested_capital = UnrealizedInvestedCapital {
            in_profit: cfg.import("invested_capital_in_profit", v0)?,
            in_loss: cfg.import("invested_capital_in_loss", v0)?,
        };

        Ok(Self {
            inner,
            gross_pnl,
            invested_capital,
            sentiment,
        })
    }

    #[inline(always)]
    pub(crate) fn push_state_all(&mut self, state: &UnrealizedState) {
        self.inner.push_state(state);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.inner.collect_vecs_mut();
        vecs.push(&mut self.gross_pnl.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital.in_profit.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital.in_loss.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.sentiment.pain_index.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.sentiment.greed_index.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.sentiment.net.cents.height as &mut dyn AnyStoredVec);
        vecs
    }

    pub(crate) fn compute_rest_all(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        supply_in_profit_sats: &(impl ReadableVec<Height, Sats> + Sync),
        supply_in_loss_sats: &(impl ReadableVec<Height, Sats> + Sync),
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_rest(starting_indexes, exit)?;

        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.inner.core.basic.profit.cents.height,
            &self.inner.core.basic.loss.cents.height,
            exit,
        )?;

        // invested_capital_in_profit = supply_profit_sats × spot / ONE_BTC - unrealized_profit
        self.invested_capital.in_profit.cents.height.compute_transform3(
            starting_indexes.height,
            supply_in_profit_sats,
            &prices.spot.cents.height,
            &self.inner.core.basic.profit.cents.height,
            |(h, supply_sats, spot, profit, ..)| {
                let market_value = supply_sats.as_u128() * spot.as_u128() / Sats::ONE_BTC_U128;
                (h, Cents::new(market_value.saturating_sub(profit.as_u128()) as u64))
            },
            exit,
        )?;

        // invested_capital_in_loss = supply_loss_sats × spot / ONE_BTC + unrealized_loss
        self.invested_capital.in_loss.cents.height.compute_transform3(
            starting_indexes.height,
            supply_in_loss_sats,
            &prices.spot.cents.height,
            &self.inner.core.basic.loss.cents.height,
            |(h, supply_sats, spot, loss, ..)| {
                let market_value = supply_sats.as_u128() * spot.as_u128() / Sats::ONE_BTC_U128;
                (h, Cents::new((market_value + loss.as_u128()) as u64))
            },
            exit,
        )?;

        self.compute_rest_extended(prices, starting_indexes, exit)?;

        self.sentiment
            .net
            .cents
            .height
            .compute_binary::<Cents, Cents, CentsSubtractToCentsSigned>(
                starting_indexes.height,
                &self.sentiment.greed_index.cents.height,
                &self.sentiment.pain_index.cents.height,
                exit,
            )?;

        Ok(())
    }

    fn compute_rest_extended(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // greed = spot - investor_price_winners
        // investor_price = investor_cap / invested_cap (both in CentsSats)
        // invested_cap is now in Cents (already divided by ONE_BTC), so multiply back
        self.sentiment.greed_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.inner.investor_cap_in_profit_raw,
            &self.invested_capital.in_profit.cents.height,
            &prices.spot.cents.height,
            |(h, investor_cap, invested_cap_cents, spot, ..)| {
                let invested_cap_raw = invested_cap_cents.as_u128() * Sats::ONE_BTC_U128;
                if invested_cap_raw == 0 {
                    return (h, Cents::ZERO);
                }
                let investor_price = investor_cap.inner() / invested_cap_raw;
                let spot_u128 = spot.as_u128();
                (h, Cents::new(spot_u128.saturating_sub(investor_price) as u64))
            },
            exit,
        )?;

        // pain = investor_price_losers - spot
        self.sentiment.pain_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.inner.investor_cap_in_loss_raw,
            &self.invested_capital.in_loss.cents.height,
            &prices.spot.cents.height,
            |(h, investor_cap, invested_cap_cents, spot, ..)| {
                let invested_cap_raw = invested_cap_cents.as_u128() * Sats::ONE_BTC_U128;
                if invested_cap_raw == 0 {
                    return (h, Cents::ZERO);
                }
                let investor_price = investor_cap.inner() / invested_cap_raw;
                let spot_u128 = spot.as_u128();
                (h, Cents::new(investor_price.saturating_sub(spot_u128) as u64))
            },
            exit,
        )?;

        Ok(())
    }

}
