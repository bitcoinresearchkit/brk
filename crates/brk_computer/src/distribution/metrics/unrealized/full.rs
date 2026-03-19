use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, CentsSquaredSats, Height, Indexes, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, BytesVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::distribution::state::UnrealizedState;
use crate::internal::{CentsSubtractToCentsSigned, FiatPerBlock};
use crate::{distribution::metrics::ImportConfig, prices};

use super::UnrealizedCore;

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
    pub inner: UnrealizedCore<M>,

    pub gross_pnl: FiatPerBlock<Cents, M>,
    pub invested_capital: UnrealizedInvestedCapital<M>,

    #[traversable(hidden)]
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    #[traversable(hidden)]
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,

    pub sentiment: UnrealizedSentiment<M>,
}

impl UnrealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let inner = UnrealizedCore::forced_import(cfg)?;

        let gross_pnl = cfg.import("unrealized_gross_pnl", v0)?;

        let invested_capital = UnrealizedInvestedCapital {
            in_profit: cfg.import("invested_capital_in_profit", v0)?,
            in_loss: cfg.import("invested_capital_in_loss", v0)?,
        };

        let investor_cap_in_profit_raw = cfg.import("investor_cap_in_profit_raw", v0)?;
        let investor_cap_in_loss_raw = cfg.import("investor_cap_in_loss_raw", v0)?;

        let sentiment = UnrealizedSentiment {
            pain_index: cfg.import("pain_index", v0)?,
            greed_index: cfg.import("greed_index", v0)?,
            net: cfg.import("net_sentiment", Version::ONE)?,
        };

        Ok(Self {
            inner,
            gross_pnl,
            invested_capital,
            investor_cap_in_profit_raw,
            investor_cap_in_loss_raw,
            sentiment,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.inner
            .min_stateful_len()
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len())
    }

    #[inline(always)]
    pub(crate) fn push_state_all(&mut self, state: &UnrealizedState) {
        self.inner.push_state(state);
        self.investor_cap_in_profit_raw
            .push(CentsSquaredSats::new(state.investor_cap_in_profit_raw));
        self.investor_cap_in_loss_raw
            .push(CentsSquaredSats::new(state.investor_cap_in_loss_raw));
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.inner.collect_vecs_mut();
        vecs.push(&mut self.gross_pnl.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital.in_profit.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital.in_loss.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec);
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

        // gross_pnl = profit + loss
        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.inner.basic.profit.cents.height,
            &self.inner.basic.loss.cents.height,
            exit,
        )?;

        // invested_capital_in_profit = supply_profit_sats × spot / ONE_BTC - unrealized_profit
        self.invested_capital.in_profit.cents.height.compute_transform3(
            starting_indexes.height,
            supply_in_profit_sats,
            &prices.spot.cents.height,
            &self.inner.basic.profit.cents.height,
            |(h, supply_sats, spot, profit, ..): (_, Sats, Cents, Cents, _)| {
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
            &self.inner.basic.loss.cents.height,
            |(h, supply_sats, spot, loss, ..): (_, Sats, Cents, Cents, _)| {
                let market_value = supply_sats.as_u128() * spot.as_u128() / Sats::ONE_BTC_U128;
                (h, Cents::new((market_value + loss.as_u128()) as u64))
            },
            exit,
        )?;

        Ok(())
    }

    /// Compute sentiment using investor_price (original formula).
    /// Called after cost_basis.in_profit/loss are computed at the cohort level.
    pub(crate) fn compute_sentiment(
        &mut self,
        starting_indexes: &Indexes,
        spot: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        // greed = spot - investor_price_winners
        // investor_price = investor_cap / invested_cap
        // invested_cap is in Cents (already / ONE_BTC), multiply back for CentsSats scale
        self.sentiment.greed_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.investor_cap_in_profit_raw,
            &self.invested_capital.in_profit.cents.height,
            spot,
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
            &self.investor_cap_in_loss_raw,
            &self.invested_capital.in_loss.cents.height,
            spot,
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

        // net = greed - pain
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
}
