use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentFromHeight, RatioDollarsBp16};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedFull};

use super::RelativeBase;

/// Full relative metrics (Source/Extended tier).
///
/// Contains all Complete-tier fields (via Deref to RelativeBase) plus:
/// - Source-only: invested_capital_in_profit/loss_rel_to_realized_cap
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: RelativeBase<M>,

    // --- Source-only fields ---
    pub invested_capital_in_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub invested_capital_in_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
}

impl RelativeFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let base = RelativeBase::forced_import(cfg)?;

        Ok(Self {
            base,
            invested_capital_in_profit_rel_to_realized_cap: cfg.import_percent_bp16(
                "invested_capital_in_profit_rel_to_realized_cap",
                Version::ZERO,
            )?,
            invested_capital_in_loss_rel_to_realized_cap: cfg.import_percent_bp16(
                "invested_capital_in_loss_rel_to_realized_cap",
                Version::ZERO,
            )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedFull,
        realized: &RealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute Complete-tier fields
        self.base.compute(
            max_from,
            &unrealized.base,
            supply_total_sats,
            market_cap,
            exit,
        )?;

        // Source-only
        self.invested_capital_in_profit_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.invested_capital_in_profit.usd.height,
                &realized.realized_cap.height,
                exit,
            )?;
        self.invested_capital_in_loss_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.invested_capital_in_loss.usd.height,
                &realized.realized_cap.height,
                exit,
            )?;
        Ok(())
    }
}
