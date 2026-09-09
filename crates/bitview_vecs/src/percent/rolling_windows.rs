use bitview_collections::Windows;
use bitview_compute::FixedRatio;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::{IndexSources, PercentPerBlock};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct PercentRollingWindows<B: FixedRatio, M: StorageMode = Rw>(
    pub Windows<PercentPerBlock<B, M>>,
);

impl<B: FixedRatio> PercentRollingWindows<B> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            PercentPerBlock::forced_import(
                cache,
                db,
                &format!("{name}_{suffix}"),
                version + Version::ONE,
                indexes,
            )
        })?))
    }
}
