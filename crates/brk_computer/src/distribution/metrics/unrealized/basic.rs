use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::{metrics::ImportConfig, state::UnrealizedState},
    internal::FiatPerBlockWithSum24h,
};

/// Basic unrealized metrics: unrealized profit/loss (fiat + 24h sums).
#[derive(Traversable)]
pub struct UnrealizedBasic<M: StorageMode = Rw> {
    pub profit: FiatPerBlockWithSum24h<Cents, M>,
    pub loss: FiatPerBlockWithSum24h<Cents, M>,
}

impl UnrealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            profit: cfg.import("unrealized_profit", v1)?,
            loss: cfg.import("unrealized_loss", v1)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.profit
            .raw
            .cents
            .height
            .len()
            .min(self.loss.raw.cents.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &UnrealizedState) -> Result<()> {
        self.profit
            .raw
            .cents
            .height
            .truncate_push(height, state.unrealized_profit)?;
        self.loss
            .raw
            .cents
            .height
            .truncate_push(height, state.unrealized_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.profit.raw.cents.height as &mut dyn AnyStoredVec,
            &mut self.loss.raw.cents.height,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; profit.raw.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.raw.cents.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.sum.compute_rolling_sum(
            max_from,
            &blocks.lookback.height_24h_ago,
            &self.profit.raw.cents.height,
            exit,
        )?;
        self.loss.sum.compute_rolling_sum(
            max_from,
            &blocks.lookback.height_24h_ago,
            &self.loss.raw.cents.height,
            exit,
        )?;

        Ok(())
    }
}
