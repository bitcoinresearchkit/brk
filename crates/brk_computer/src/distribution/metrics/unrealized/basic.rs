use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{metrics::ImportConfig, state::UnrealizedState},
    internal::FiatPerBlockCumulativeWithSums,
};

use super::UnrealizedMinimal;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedBasic<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: UnrealizedMinimal<M>,
    pub profit: FiatPerBlockCumulativeWithSums<Cents, M>,
    pub loss: FiatPerBlockCumulativeWithSums<Cents, M>,
}

impl UnrealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            minimal: UnrealizedMinimal::forced_import(cfg)?,
            profit: cfg.import("unrealized_profit", v1)?,
            loss: cfg.import("unrealized_loss", v1)?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.profit
            .base
            .cents
            .height
            .len()
            .min(self.loss.base.cents.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &UnrealizedState) -> Result<()> {
        self.profit
            .base
            .cents
            .height
            .truncate_push(height, state.unrealized_profit)?;
        self.loss
            .base
            .cents
            .height
            .truncate_push(height, state.unrealized_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.profit.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.loss.base.cents.height,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; profit.base.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.base.cents.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.profit.compute_rest(max_from, exit)?;
        self.loss.compute_rest(max_from, exit)?;
        Ok(())
    }
}
