use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{metrics::ImportConfig, state::UnrealizedState},
    internal::{FiatPerBlockCumulativeWithSums, LazyPerBlock, NegCentsUnsignedToDollars},
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
    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: LazyPerBlock<Dollars, Cents>,
}

impl UnrealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let loss: FiatPerBlockCumulativeWithSums<Cents> = cfg.import("unrealized_loss", v1)?;

        let neg_loss = LazyPerBlock::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            loss.base.cents.height.read_only_boxed_clone(),
            &loss.base.cents,
        );

        Ok(Self {
            minimal: UnrealizedMinimal::forced_import(cfg)?,
            profit: cfg.import("unrealized_profit", v1)?,
            loss,
            neg_loss,
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
