use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Rw, StorageMode};

use crate::distribution::metrics::ImportConfig;

use super::{CostBasisBase, CostBasisExtended};

/// Cost basis metrics with guaranteed extended (no Option).
#[derive(Deref, DerefMut, Traversable)]
pub struct CostBasisWithExtended<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: CostBasisBase<M>,
    #[traversable(flatten)]
    pub extended: CostBasisExtended<M>,
}

impl CostBasisWithExtended {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: CostBasisBase::forced_import(cfg)?,
            extended: CostBasisExtended::forced_import(cfg)?,
        })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.base.collect_vecs_mut();
        vecs.extend(self.extended.collect_vecs_mut());
        vecs
    }
}
