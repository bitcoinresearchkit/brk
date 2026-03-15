use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, CentsSigned, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::UnrealizedState,
    },
    internal::{CentsSubtractToCentsSigned, FiatPerBlock},
};

use super::UnrealizedBasic;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub basic: UnrealizedBasic<M>,

    pub net_pnl: FiatPerBlock<CentsSigned, M>,
}

impl UnrealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let basic = UnrealizedBasic::forced_import(cfg)?;
        let net_unrealized_pnl = cfg.import("net_unrealized_pnl", Version::ZERO)?;

        Ok(Self {
            basic,
            net_pnl: net_unrealized_pnl,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.basic.min_stateful_len()
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &UnrealizedState) {
        self.basic.push_state(state);
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
                &self.basic.profit.base.cents.height,
                &self.basic.loss.base.cents.height,
                exit,
            )?;

        Ok(())
    }
}
