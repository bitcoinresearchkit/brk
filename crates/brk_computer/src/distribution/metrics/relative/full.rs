use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Dollars, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::{ImportConfig, UnrealizedCore},
    internal::{PercentPerBlock, RatioDollarsBp16, RatioSatsBp16},
};

/// Full relative metrics (sth/lth/all tier).
#[derive(Traversable)]
pub struct RelativeFull<M: StorageMode = Rw> {
    #[traversable(wrap = "unrealized/profit/supply", rename = "rel_to_own_supply")]
    pub supply_in_profit_rel_to_own_supply: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "unrealized/loss/supply", rename = "rel_to_own_supply")]
    pub supply_in_loss_rel_to_own_supply: PercentPerBlock<BasisPoints16, M>,

    #[traversable(wrap = "unrealized/profit", rename = "rel_to_market_cap")]
    pub unrealized_profit_rel_to_market_cap: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "unrealized/loss", rename = "rel_to_market_cap")]
    pub unrealized_loss_rel_to_market_cap: PercentPerBlock<BasisPoints16, M>,
}

impl RelativeFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import("supply_in_profit_rel_to_own_supply", v1)?,
            supply_in_loss_rel_to_own_supply: cfg.import("supply_in_loss_rel_to_own_supply", v1)?,
            unrealized_profit_rel_to_market_cap: cfg
                .import("unrealized_profit_rel_to_market_cap", v2)?,
            unrealized_loss_rel_to_market_cap: cfg
                .import("unrealized_loss_rel_to_market_cap", v2)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedCore,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &unrealized.supply_in_profit.sats.height,
                supply_total_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &unrealized.supply_in_loss.sats.height,
                supply_total_sats,
                exit,
            )?;

        self.unrealized_profit_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.profit.raw.usd.height,
                market_cap,
                exit,
            )?;
        self.unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.loss.raw.usd.height,
                market_cap,
                exit,
            )?;
        Ok(())
    }
}
