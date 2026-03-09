use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSats, CentsSigned, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode, WritableVec};

use crate::distribution::state::UnrealizedState;
use crate::internal::{CentsSubtractToCentsSigned, FiatPerBlock};
use crate::{distribution::metrics::ImportConfig, prices};

use super::UnrealizedBase;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: UnrealizedBase<M>,

    pub gross_pnl: FiatPerBlock<Cents, M>,
    pub invested_capital_in_profit: FiatPerBlock<Cents, M>,
    pub invested_capital_in_loss: FiatPerBlock<Cents, M>,

    pub pain_index: FiatPerBlock<Cents, M>,
    pub greed_index: FiatPerBlock<Cents, M>,
    pub net_sentiment: FiatPerBlock<CentsSigned, M>,
}

impl UnrealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let inner = UnrealizedBase::forced_import(cfg)?;

        let gross_pnl = cfg.import("unrealized_gross_pnl", v0)?;
        let invested_capital_in_profit = cfg.import("invested_capital_in_profit", v0)?;
        let invested_capital_in_loss = cfg.import("invested_capital_in_loss", v0)?;

        let pain_index = cfg.import("pain_index", v0)?;
        let greed_index = cfg.import("greed_index", v0)?;
        let net_sentiment = cfg.import("net_sentiment", Version::ONE)?;

        Ok(Self {
            inner,
            gross_pnl,
            invested_capital_in_profit,
            invested_capital_in_loss,
            pain_index,
            greed_index,
            net_sentiment,
        })
    }

    pub(crate) fn truncate_push_all(
        &mut self,
        height: Height,
        state: &UnrealizedState,
    ) -> Result<()> {
        self.inner.truncate_push(height, state)?;
        self.invested_capital_in_profit
            .cents
            .height
            .truncate_push(height, state.invested_capital_in_profit)?;
        self.invested_capital_in_loss
            .cents
            .height
            .truncate_push(height, state.invested_capital_in_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.inner.collect_vecs_mut();
        vecs.push(&mut self.gross_pnl.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_profit.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_loss.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.pain_index.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.greed_index.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.net_sentiment.cents.height as &mut dyn AnyStoredVec);
        vecs
    }

    pub(crate) fn compute_rest_all(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_rest(prices, starting_indexes, exit)?;

        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.inner.core.profit.cents.height,
            &self.inner.core.loss.cents.height,
            exit,
        )?;

        self.invested_capital_in_profit
            .cents
            .height
            .compute_transform(
                starting_indexes.height,
                &self.inner.invested_capital_in_profit_raw,
                |(h, raw, ..)| (h, CentsSats::to_cents(raw)),
                exit,
            )?;

        self.invested_capital_in_loss
            .cents
            .height
            .compute_transform(
                starting_indexes.height,
                &self.inner.invested_capital_in_loss_raw,
                |(h, raw, ..)| (h, CentsSats::to_cents(raw)),
                exit,
            )?;

        self.compute_rest_extended(prices, starting_indexes, exit)?;
        Ok(())
    }

    fn compute_rest_extended(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.pain_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.inner.investor_cap_in_loss_raw,
            &self.inner.invested_capital_in_loss_raw,
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

        self.greed_index.cents.height.compute_transform3(
            starting_indexes.height,
            &self.inner.investor_cap_in_profit_raw,
            &self.inner.invested_capital_in_profit_raw,
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
