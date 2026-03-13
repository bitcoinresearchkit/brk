use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned32, Dollars, Height, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentPerBlock, RatioDollarsBp16, RatioDollarsBps32};

use crate::distribution::metrics::{ImportConfig, UnrealizedCore};

/// Extended relative metrics for own total unrealized PnL (extended only).
#[derive(Traversable)]
pub struct RelativeExtendedOwnPnl<M: StorageMode = Rw> {
    #[traversable(wrap = "unrealized/profit", rename = "rel_to_own_gross")]
    pub unrealized_profit_rel_to_own_gross_pnl: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "unrealized/loss", rename = "rel_to_own_gross")]
    pub unrealized_loss_rel_to_own_gross_pnl: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "unrealized/net_pnl", rename = "rel_to_own_gross")]
    pub net_unrealized_pnl_rel_to_own_gross_pnl: PercentPerBlock<BasisPointsSigned32, M>,

}

impl RelativeExtendedOwnPnl {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            unrealized_profit_rel_to_own_gross_pnl: cfg
                .import("unrealized_profit_rel_to_own_gross_pnl", v1)?,
            unrealized_loss_rel_to_own_gross_pnl: cfg
                .import("unrealized_loss_rel_to_own_gross_pnl", v1)?,
            net_unrealized_pnl_rel_to_own_gross_pnl: cfg
                .import("net_unrealized_pnl_rel_to_own_gross_pnl", Version::new(3))?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedCore,
        gross_pnl_usd: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_profit_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.profit.base.usd.height,
                gross_pnl_usd,
                exit,
            )?;
        self.unrealized_loss_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.loss.base.usd.height,
                gross_pnl_usd,
                exit,
            )?;
        self.net_unrealized_pnl_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBps32>(
                max_from,
                &unrealized.net_pnl.usd.height,
                gross_pnl_usd,
                exit,
            )?;
        Ok(())
    }
}
