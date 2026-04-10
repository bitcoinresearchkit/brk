use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadOnlyClone, Rw, StorageMode};

use crate::{
    indexes,
    internal::{NumericValue, Resolutions},
};

/// Like [`PerBlock`](super::PerBlock) but with height wrapped in [`CachedVec`]
/// for fast repeated reads.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct CachedPerBlock<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: CachedVec<M::Stored<EagerVec<PcoVec<Height, T>>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<T> CachedPerBlock<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;

        let resolutions =
            Resolutions::forced_import(name, height.read_only_clone(), version, indexes);

        Ok(Self {
            height: CachedVec::wrap(height),
            resolutions: Box::new(resolutions),
        })
    }
}
