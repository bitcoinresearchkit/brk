use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, Height, Version};
use vecdb::{
    CachedVec, Database, EagerVec, ImportableVec, PcoVec, PcoVecValue, Rw, StorageMode, VecIndex,
};

/// Resolution with cached first-height lookup.
#[derive(Traversable)]
pub struct ResolutionVecs<I: VecIndex + PcoVecValue, M: StorageMode = Rw> {
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<I, Height>>>>,
}

impl<I: VecIndex + PcoVecValue> ResolutionVecs<I> {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}

/// Resolution with date and cached first-height lookup.
#[derive(Traversable)]
pub struct DatedResolutionVecs<I: VecIndex + PcoVecValue, M: StorageMode = Rw> {
    pub date: M::Stored<EagerVec<PcoVec<I, Date>>>,
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<I, Height>>>>,
}

impl<I: VecIndex + PcoVecValue> DatedResolutionVecs<I> {
    pub(crate) fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}
