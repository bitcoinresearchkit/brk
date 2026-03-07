use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned32, Dollars, Height, Version};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{PercentFromHeight, RatioDollarsBp16, RatioDollarsBps32};

use crate::distribution::metrics::{ImportConfig, UnrealizedFull};

/// Extended relative metrics for own total unrealized PnL (extended only).
#[derive(Traversable)]
pub struct RelativeExtendedOwnPnl<M: StorageMode = Rw> {
    pub unrealized_profit_rel_to_own_gross_pnl: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_own_gross_pnl: PercentFromHeight<BasisPoints16, M>,
    pub net_unrealized_pnl_rel_to_own_gross_pnl: PercentFromHeight<BasisPointsSigned32, M>,
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
        unrealized: &UnrealizedFull,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_profit_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_profit.usd.height,
                &unrealized.gross_pnl.usd.height,
                exit,
            )?;
        self.unrealized_loss_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                &unrealized.gross_pnl.usd.height,
                exit,
            )?;
        self.net_unrealized_pnl_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBps32>(
                max_from,
                &unrealized.net_unrealized_pnl.usd.height,
                &unrealized.gross_pnl.usd.height,
                exit,
            )?;
        Ok(())
    }
}
