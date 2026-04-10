use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, Height, Version};
use vecdb::{
    CachedVec, Database, EagerVec, ImportableVec, PcoVec, PcoVecValue, Rw, StorageMode, VecIndex,
};

/// Resolution with identity mapping and cached first-height lookup.
#[derive(Traversable)]
pub struct ResolutionVecs<I: VecIndex + PcoVecValue, M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<I, I>>>,
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<I, Height>>>>,
}

/// Resolution with both identity and first-height cached (halving, epoch).
#[derive(Traversable)]
pub struct CachedResolutionVecs<I: VecIndex + PcoVecValue, M: StorageMode = Rw> {
    pub identity: CachedVec<M::Stored<EagerVec<PcoVec<I, I>>>>,
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<I, Height>>>>,
}

impl<I: VecIndex + PcoVecValue> CachedResolutionVecs<I> {
    pub(crate) fn forced_import(
        db: &Database,
        identity_name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            identity: CachedVec::wrap(EagerVec::forced_import(db, identity_name, version)?),
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}

impl<I: VecIndex + PcoVecValue> ResolutionVecs<I> {
    pub(crate) fn forced_import(
        db: &Database,
        identity_name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, identity_name, version)?,
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}

/// Resolution with identity, date, and cached first-height lookup.
#[derive(Traversable)]
pub struct DatedResolutionVecs<I: VecIndex + PcoVecValue, M: StorageMode = Rw> {
    pub identity: M::Stored<EagerVec<PcoVec<I, I>>>,
    pub date: M::Stored<EagerVec<PcoVec<I, Date>>>,
    pub first_height: CachedVec<M::Stored<EagerVec<PcoVec<I, Height>>>>,
}

impl<I: VecIndex + PcoVecValue> DatedResolutionVecs<I> {
    pub(crate) fn forced_import(
        db: &Database,
        identity_name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            identity: EagerVec::forced_import(db, identity_name, version)?,
            date: EagerVec::forced_import(db, "date", version)?,
            first_height: CachedVec::wrap(EagerVec::forced_import(db, "first_height", version)?),
        })
    }
}
