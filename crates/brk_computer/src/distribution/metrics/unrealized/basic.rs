use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{metrics::ImportConfig, state::UnrealizedState},
    internal::{FiatPerBlock, LazyPerBlock, NegCentsUnsignedToDollars},
};

use super::UnrealizedMinimal;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedBasic<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: UnrealizedMinimal<M>,
    pub profit: FiatPerBlock<Cents, M>,
    pub loss: FiatPerBlock<Cents, M>,
    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: LazyPerBlock<Dollars, Cents>,
}

impl UnrealizedBasic {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let loss: FiatPerBlock<Cents> = cfg.import("unrealized_loss", v1)?;

        let neg_loss = LazyPerBlock::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("unrealized_loss_neg"),
            cfg.version,
            loss.cents.height.read_only_boxed_clone(),
            &loss.cents,
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
            .cents
            .height
            .len()
            .min(self.loss.cents.height.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &UnrealizedState) {
        self.profit
            .cents
            .height
            .push(state.unrealized_profit);
        self.loss
            .cents
            .height
            .push(state.unrealized_loss);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.profit.cents.height as &mut dyn AnyStoredVec,
            &mut self.loss.cents.height,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; profit.cents.height);
        sum_others!(self, starting_indexes, others, exit; loss.cents.height);
        Ok(())
    }
}
