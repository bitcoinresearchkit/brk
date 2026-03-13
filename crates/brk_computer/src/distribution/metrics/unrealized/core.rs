use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::UnrealizedState,
    },
    internal::{CentsSubtractToCentsSigned, FiatPerBlock, LazyPerBlock, NegCentsUnsignedToDollars},
};

use brk_types::Dollars;

use super::UnrealizedBasic;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub basic: UnrealizedBasic<M>,

    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: LazyPerBlock<Dollars, Cents>,
    pub net_pnl: FiatPerBlock<CentsSigned, M>,
}

impl UnrealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let basic = UnrealizedBasic::forced_import(cfg)?;

        let neg_unrealized_loss = LazyPerBlock::from_computed::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            basic.loss.raw.cents.height.read_only_boxed_clone(),
            &basic.loss.raw.cents,
        );

        let net_unrealized_pnl = cfg.import("net_unrealized_pnl", Version::ZERO)?;

        Ok(Self {
            basic,
            neg_loss: neg_unrealized_loss,
            net_pnl: net_unrealized_pnl,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.basic.min_stateful_len()
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        height_state: &UnrealizedState,
    ) -> Result<()> {
        self.basic.truncate_push(height, height_state)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.basic.collect_vecs_mut()
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let basic_refs: Vec<&UnrealizedBasic> = others.iter().map(|o| &o.basic).collect();
        self.basic
            .compute_from_sources(starting_indexes, &basic_refs, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.basic
            .compute_rest(starting_indexes.height, exit)?;

        self.net_pnl
            .cents
            .height
            .compute_binary::<Cents, Cents, CentsSubtractToCentsSigned>(
                starting_indexes.height,
                &self.basic.profit.raw.cents.height,
                &self.basic.loss.raw.cents.height,
                exit,
            )?;

        Ok(())
    }
}
