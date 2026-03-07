use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentFromHeight, RatioSatsBp16};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Relative metrics for the Complete tier.
#[derive(Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    pub supply_in_profit_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import("supply_in_profit_rel_to_own_supply", v1)?,
            supply_in_loss_rel_to_own_supply: cfg
                .import("supply_in_loss_rel_to_own_supply", v1)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
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
        Ok(())
    }
}
