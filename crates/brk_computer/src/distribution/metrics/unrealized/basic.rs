use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::{metrics::ImportConfig, state::UnrealizedState},
    internal::FiatPerBlockWithSum24h,
    prices,
};

use super::UnrealizedMinimal;

/// Basic unrealized metrics: supply in profit/loss + unrealized profit/loss (fiat + 24h sums).
#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedBasic<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: UnrealizedMinimal<M>,

    pub profit: FiatPerBlockWithSum24h<Cents, M>,
    pub loss: FiatPerBlockWithSum24h<Cents, M>,
}

impl UnrealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let minimal = UnrealizedMinimal::forced_import(cfg)?;

        Ok(Self {
            minimal,
            profit: cfg.import("unrealized_profit", v1)?,
            loss: cfg.import("unrealized_loss", v1)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.minimal
            .min_stateful_height_len()
            .min(self.profit.raw.cents.height.len())
            .min(self.loss.raw.cents.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &UnrealizedState) -> Result<()> {
        self.minimal.truncate_push(height, state)?;
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
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.profit.raw.cents.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.loss.raw.cents.height);
        vecs
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let minimal_refs: Vec<&UnrealizedMinimal> = others.iter().map(|o| &o.minimal).collect();
        self.minimal
            .compute_from_sources(starting_indexes, &minimal_refs, exit)?;
        sum_others!(self, starting_indexes, others, exit; profit.raw.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.raw.cents.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal.compute_rest(prices, max_from, exit)?;

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
