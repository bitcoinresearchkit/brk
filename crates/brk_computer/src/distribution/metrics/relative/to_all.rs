use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentPerBlock, RatioSatsBp16};

use crate::distribution::metrics::{ImportConfig, SupplyCore};

/// Relative-to-all metrics (not present for the "all" cohort itself).
#[derive(Traversable)]
pub struct RelativeToAll<M: StorageMode = Rw> {
    #[traversable(wrap = "supply", rename = "rel_to_circulating_supply")]
    pub supply_rel_to_circulating_supply: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "supply/in_profit", rename = "rel_to_circulating_supply")]
    pub supply_in_profit_rel_to_circulating_supply: PercentPerBlock<BasisPoints16, M>,
    #[traversable(wrap = "supply/in_loss", rename = "rel_to_circulating_supply")]
    pub supply_in_loss_rel_to_circulating_supply: PercentPerBlock<BasisPoints16, M>,
}

impl RelativeToAll {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            supply_rel_to_circulating_supply: cfg
                .import("supply_rel_to_circulating_supply", Version::ONE)?,
            supply_in_profit_rel_to_circulating_supply: cfg
                .import("supply_in_profit_rel_to_circulating_supply", Version::ONE)?,
            supply_in_loss_rel_to_circulating_supply: cfg
                .import("supply_in_loss_rel_to_circulating_supply", Version::ONE)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        supply: &SupplyCore,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &supply.total.sats.height,
                all_supply_sats,
                exit,
            )?;
        self.supply_in_profit_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &supply.in_profit.sats.height,
                all_supply_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &supply.in_loss.sats.height,
                all_supply_sats,
                exit,
            )?;
        Ok(())
    }
}
