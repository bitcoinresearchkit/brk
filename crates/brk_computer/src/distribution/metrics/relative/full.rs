use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Dollars, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::{
    distribution::metrics::{ImportConfig, SupplyCore, UnrealizedBasic},
    internal::{PercentPerBlock, RatioDollarsBp16, RatioSatsBp16},
};

/// Full relative metrics (sth/lth/all tier).
#[derive(Traversable)]
pub struct RelativeFull<M: StorageMode = Rw> {
    #[traversable(wrap = "supply/in_profit", rename = "to_own")]
    pub supply_in_profit_to_own: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "supply/in_loss", rename = "to_own")]
    pub supply_in_loss_to_own: PercentPerBlock<BasisPoints16, M>,

    #[traversable(wrap = "unrealized/profit", rename = "to_mcap")]
    pub unrealized_profit_to_mcap: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "unrealized/loss", rename = "to_mcap")]
    pub unrealized_loss_to_mcap: PercentPerBlock<BasisPoints16, M>,
}

impl RelativeFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        Ok(Self {
            supply_in_profit_to_own: cfg.import("supply_in_profit_to_own", v1)?,
            supply_in_loss_to_own: cfg.import("supply_in_loss_to_own", v1)?,
            unrealized_profit_to_mcap: cfg.import("unrealized_profit_to_mcap", v2)?,
            unrealized_loss_to_mcap: cfg.import("unrealized_loss_to_mcap", v2)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        supply: &SupplyCore,
        unrealized: &UnrealizedBasic,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit_to_own
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &supply.in_profit.sats.height,
                &supply.total.sats.height,
                exit,
            )?;
        self.supply_in_loss_to_own
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &supply.in_loss.sats.height,
                &supply.total.sats.height,
                exit,
            )?;

        self.unrealized_profit_to_mcap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.profit.usd.height,
                market_cap,
                exit,
            )?;
        self.unrealized_loss_to_mcap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.loss.usd.height,
                market_cap,
                exit,
            )?;
        Ok(())
    }
}
