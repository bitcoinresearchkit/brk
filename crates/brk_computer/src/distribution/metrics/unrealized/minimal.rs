use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::prices;

use crate::internal::AmountPerBlock;

use crate::distribution::{metrics::ImportConfig, state::UnrealizedState};

/// Minimal unrealized metrics: supply in profit/loss only.
#[derive(Traversable)]
pub struct UnrealizedMinimal<M: StorageMode = Rw> {
    #[traversable(wrap = "profit", rename = "supply")]
    pub supply_in_profit: AmountPerBlock<M>,
    #[traversable(wrap = "loss", rename = "supply")]
    pub supply_in_loss: AmountPerBlock<M>,
}

impl UnrealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        Ok(Self {
            supply_in_profit: cfg.import("supply_in_profit", v0)?,
            supply_in_loss: cfg.import("supply_in_loss", v0)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &UnrealizedState) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .truncate_push(height, state.supply_in_profit)?;
        self.supply_in_loss
            .sats
            .height
            .truncate_push(height, state.supply_in_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.cents.height,
            &mut self.supply_in_loss.sats.height,
            &mut self.supply_in_loss.cents.height,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; supply_in_profit.sats.height);
        sum_others!(self, starting_indexes, others, exit; supply_in_loss.sats.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.compute(prices, max_from, exit)?;
        self.supply_in_loss.compute(prices, max_from, exit)?;
        Ok(())
    }
}
