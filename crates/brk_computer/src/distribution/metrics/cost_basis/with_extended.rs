use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{Rw, StorageMode};

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

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.extended.validate_computed_versions(base_version)
    }
}
