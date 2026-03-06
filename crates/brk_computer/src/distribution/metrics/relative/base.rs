use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned32, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{NegRatioDollarsBps32, PercentFromHeight, RatioDollarsBp16};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

use super::RelativeComplete;

/// Full relative metrics (Source/Extended tier).
///
/// Contains all Complete-tier fields (via Deref to RelativeComplete) plus:
/// - Source-only: neg_unrealized_loss_rel_to_market_cap, invested_capital_in_profit/loss_rel_to_realized_cap
#[derive(Deref, DerefMut, Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub complete: RelativeComplete<M>,

    // --- Source-only fields ---
    pub neg_unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub invested_capital_in_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub invested_capital_in_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let complete = RelativeComplete::forced_import(cfg)?;

        Ok(Self {
            complete,
            neg_unrealized_loss_rel_to_market_cap: cfg
                .import_percent_bps32("neg_unrealized_loss_rel_to_market_cap", Version::new(3))?,
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
        unrealized: &UnrealizedBase,
        realized: &RealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute Complete-tier fields
        self.complete.compute(
            max_from,
            &unrealized.complete,
            supply_total_sats,
            market_cap,
            exit,
        )?;

        // Source-only
        self.neg_unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, NegRatioDollarsBps32>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                market_cap,
                exit,
            )?;
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
