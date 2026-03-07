use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32, BasisPointsSigned32, Dollars, Height, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentFromHeight, RatioDollarsBp16, RatioDollarsBp32, RatioDollarsBps32};

use crate::distribution::metrics::{ImportConfig, UnrealizedCore};

/// Extended relative metrics for own market cap (extended && rel_to_all).
#[derive(Traversable)]
pub struct RelativeExtendedOwnMarketCap<M: StorageMode = Rw> {
    pub unrealized_profit_rel_to_own_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_own_market_cap: PercentFromHeight<BasisPoints32, M>,
    pub net_unrealized_pnl_rel_to_own_market_cap: PercentFromHeight<BasisPointsSigned32, M>,
}

impl RelativeExtendedOwnMarketCap {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v2 = Version::new(2);

        Ok(Self {
            unrealized_profit_rel_to_own_market_cap: cfg
                .import("unrealized_profit_rel_to_own_market_cap", v2)?,
            unrealized_loss_rel_to_own_market_cap: cfg
                .import("unrealized_loss_rel_to_own_market_cap", Version::new(3))?,
            net_unrealized_pnl_rel_to_own_market_cap: cfg
                .import("net_unrealized_pnl_rel_to_own_market_cap", Version::new(3))?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedCore,
        own_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_profit_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_profit.usd.height,
                own_market_cap,
                exit,
            )?;
        self.unrealized_loss_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                own_market_cap,
                exit,
            )?;
        self.net_unrealized_pnl_rel_to_own_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBps32>(
                max_from,
                &unrealized.net_unrealized_pnl.usd.height,
                own_market_cap,
                exit,
            )?;
        Ok(())
    }
}
