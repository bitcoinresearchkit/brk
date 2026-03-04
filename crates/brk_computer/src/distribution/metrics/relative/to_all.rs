use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentFromHeight, RatioSatsBp16};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Relative-to-all metrics (not present for the "all" cohort itself).
#[derive(Traversable)]
pub struct RelativeToAll<M: StorageMode = Rw> {
    pub supply_rel_to_circulating_supply:
        PercentFromHeight<BasisPoints16, M>,
    pub supply_in_profit_rel_to_circulating_supply:
        PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_circulating_supply:
        PercentFromHeight<BasisPoints16, M>,
}

impl RelativeToAll {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            supply_rel_to_circulating_supply:
                cfg.import_percent_bp16("supply_rel_to_circulating_supply", Version::ONE)?,
            supply_in_profit_rel_to_circulating_supply:
                cfg.import_percent_bp16("supply_in_profit_rel_to_circulating_supply", Version::ONE)?,
            supply_in_loss_rel_to_circulating_supply:
                cfg.import_percent_bp16("supply_in_loss_rel_to_circulating_supply", Version::ONE)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from, supply_total_sats, all_supply_sats, exit,
            )?;
        self.supply_in_profit_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from, &unrealized.supply_in_profit.sats.height, all_supply_sats, exit,
            )?;
        self.supply_in_loss_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from, &unrealized.supply_in_loss.sats.height, all_supply_sats, exit,
            )?;
        Ok(())
    }
}
