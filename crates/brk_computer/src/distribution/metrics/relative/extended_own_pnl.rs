use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast, NegPercentageDollarsF32, PercentageDollarsF32,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Extended relative metrics for own total unrealized PnL (extended only).
#[derive(Traversable)]
pub struct RelativeExtendedOwnPnl<M: StorageMode = Rw> {
    pub unrealized_profit_rel_to_own_total_unrealized_pnl:
        ComputedFromHeightLast<StoredF32, M>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl:
        ComputedFromHeightLast<StoredF32, M>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        ComputedFromHeightLast<StoredF32, M>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        ComputedFromHeightLast<StoredF32, M>,
}

impl RelativeExtendedOwnPnl {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
    ) -> Result<Self> {
        let v1 = brk_types::Version::ONE;
        let v2 = brk_types::Version::new(2);

        Ok(Self {
            unrealized_profit_rel_to_own_total_unrealized_pnl:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            unrealized_loss_rel_to_own_total_unrealized_pnl:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    cfg.indexes,
                )?,
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
                ComputedFromHeightLast::forced_import(
                    cfg.db,
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
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
        self.unrealized_profit_rel_to_own_total_unrealized_pnl
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.unrealized_profit.height, &unrealized.total_unrealized_pnl.height, exit,
            )?;
        self.unrealized_loss_rel_to_own_total_unrealized_pnl
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.unrealized_loss.height, &unrealized.total_unrealized_pnl.height, exit,
            )?;
        self.neg_unrealized_loss_rel_to_own_total_unrealized_pnl
            .compute_binary::<Dollars, Dollars, NegPercentageDollarsF32>(
                max_from, &unrealized.unrealized_loss.height, &unrealized.total_unrealized_pnl.height, exit,
            )?;
        self.net_unrealized_pnl_rel_to_own_total_unrealized_pnl
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.net_unrealized_pnl.height, &unrealized.total_unrealized_pnl.height, exit,
            )?;
        Ok(())
    }
}
