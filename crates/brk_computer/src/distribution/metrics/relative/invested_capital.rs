use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, Height, Version};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{PercentPerBlock, RatioCentsBp16};

use crate::distribution::metrics::{ImportConfig, RealizedFull, UnrealizedFull};

/// Shares of invested capital in profit / in loss relative to own realized cap.
/// Present for cohorts with `UnrealizedFull` (all, sth, lth).
#[derive(Traversable)]
pub struct RelativeInvestedCapital<M: StorageMode = Rw> {
    #[traversable(wrap = "invested_capital/in_profit", rename = "share")]
    pub in_profit_share: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "invested_capital/in_loss", rename = "share")]
    pub in_loss_share: PercentPerBlock<BasisPoints16, M>,
}

impl RelativeInvestedCapital {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        Ok(Self {
            in_profit_share: cfg.import("invested_capital_in_profit_share", v0)?,
            in_loss_share: cfg.import("invested_capital_in_loss_share", v0)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedFull,
        realized: &RealizedFull,
        exit: &Exit,
    ) -> Result<()> {
        let realized_cap = &realized.core.minimal.cap.cents.height;
        self.in_profit_share
            .compute_binary::<Cents, Cents, RatioCentsBp16>(
                max_from,
                &unrealized.invested_capital.in_profit.cents.height,
                realized_cap,
                exit,
            )?;
        self.in_loss_share
            .compute_binary::<Cents, Cents, RatioCentsBp16>(
                max_from,
                &unrealized.invested_capital.in_loss.cents.height,
                realized_cap,
                exit,
            )?;
        Ok(())
    }
}
