use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned16, Dollars, Height};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{
    Bp16ToFloat, Bp16ToPercent, Bps16ToFloat, Bps16ToPercent,
    NegRatioDollarsBps16, PercentFromHeight, RatioDollarsBp16, RatioDollarsBps16,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Extended relative metrics for own total unrealized PnL (extended only).
#[derive(Traversable)]
pub struct RelativeExtendedOwnPnl<M: StorageMode = Rw> {
    pub unrealized_profit_rel_to_own_gross_pnl:
        PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_own_gross_pnl:
        PercentFromHeight<BasisPoints16, M>,
    pub neg_unrealized_loss_rel_to_own_gross_pnl:
        PercentFromHeight<BasisPointsSigned16, M>,
    pub net_unrealized_pnl_rel_to_own_gross_pnl:
        PercentFromHeight<BasisPointsSigned16, M>,
}

impl RelativeExtendedOwnPnl {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
    ) -> Result<Self> {
        let v1 = brk_types::Version::ONE;
        let v2 = brk_types::Version::new(2);

        Ok(Self {
            unrealized_profit_rel_to_own_gross_pnl:
                PercentFromHeight::forced_import::<Bp16ToFloat, Bp16ToPercent>(
                    cfg.db,
                    &cfg.name("unrealized_profit_rel_to_own_gross_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            unrealized_loss_rel_to_own_gross_pnl:
                PercentFromHeight::forced_import::<Bp16ToFloat, Bp16ToPercent>(
                    cfg.db,
                    &cfg.name("unrealized_loss_rel_to_own_gross_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            neg_unrealized_loss_rel_to_own_gross_pnl:
                PercentFromHeight::forced_import::<Bps16ToFloat, Bps16ToPercent>(
                    cfg.db,
                    &cfg.name("neg_unrealized_loss_rel_to_own_gross_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            net_unrealized_pnl_rel_to_own_gross_pnl:
                PercentFromHeight::forced_import::<Bps16ToFloat, Bps16ToPercent>(
                    cfg.db,
                    &cfg.name("net_unrealized_pnl_rel_to_own_gross_pnl"),
                    cfg.version + v2,
                    cfg.indexes,
                )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_profit_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.unrealized_profit.usd.height, &unrealized.gross_pnl.usd.height, exit,
            )?;
        self.unrealized_loss_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.unrealized_loss.usd.height, &unrealized.gross_pnl.usd.height, exit,
            )?;
        self.neg_unrealized_loss_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, NegRatioDollarsBps16>(
                max_from, &unrealized.unrealized_loss.usd.height, &unrealized.gross_pnl.usd.height, exit,
            )?;
        self.net_unrealized_pnl_rel_to_own_gross_pnl
            .compute_binary::<Dollars, Dollars, RatioDollarsBps16>(
                max_from, &unrealized.net_unrealized_pnl.usd.height, &unrealized.gross_pnl.usd.height, exit,
            )?;
        Ok(())
    }
}
