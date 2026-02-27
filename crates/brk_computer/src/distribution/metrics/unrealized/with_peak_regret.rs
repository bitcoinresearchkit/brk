use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use vecdb::{Rw, StorageMode};

use crate::distribution::metrics::ImportConfig;

use super::{UnrealizedBase, UnrealizedPeakRegret};

/// Unrealized metrics with guaranteed peak regret (no Option).
#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedWithPeakRegret<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: UnrealizedBase<M>,
    #[traversable(flatten)]
    pub peak_regret_ext: UnrealizedPeakRegret<M>,
}

impl UnrealizedWithPeakRegret {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: UnrealizedBase::forced_import(cfg)?,
            peak_regret_ext: UnrealizedPeakRegret::forced_import(cfg)?,
        })
    }
}
