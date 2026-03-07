use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{CentsSubtractToCentsSigned, FiatFromHeight};
use crate::prices;

use crate::distribution::metrics::ImportConfig;

use super::UnrealizedBase;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: UnrealizedBase<M>,

    pub pain_index: FiatFromHeight<Cents, M>,
    pub greed_index: FiatFromHeight<Cents, M>,
    pub net_sentiment: FiatFromHeight<CentsSigned, M>,
}

impl UnrealizedFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let inner = UnrealizedBase::forced_import(cfg)?;

        let pain_index = cfg.import("pain_index", Version::ZERO)?;
        let greed_index = cfg.import("greed_index", Version::ZERO)?;
        let net_sentiment = cfg.import("net_sentiment", Version::ONE)?;

        Ok(Self {
            inner,
            pain_index,
            greed_index,
            net_sentiment,
        })
    }

    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_rest(prices, starting_indexes, exit)?;

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
