use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Sats, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{PercentPerBlock, RatioSatsBp16};

use crate::distribution::metrics::ImportConfig;

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
        supply_in_profit_sats: &impl ReadableVec<Height, Sats>,
        supply_in_loss_sats: &impl ReadableVec<Height, Sats>,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                supply_total_sats,
                all_supply_sats,
                exit,
            )?;
        self.supply_in_profit_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                supply_in_profit_sats,
                all_supply_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_circulating_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                supply_in_loss_sats,
                all_supply_sats,
                exit,
            )?;
        Ok(())
    }
}
