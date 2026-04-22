use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Version};
use vecdb::{
    Database, EagerVec, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw, StorageMode,
};

use super::FiatType;

/// Raw per-block fiat data: cents (stored) + usd (lazy), no resolutions.
#[derive(Traversable)]
pub struct FiatBlock<C: FiatType, M: StorageMode = Rw> {
    pub usd: LazyVecFrom1<Height, Dollars, Height, C>,
    pub cents: M::Stored<EagerVec<PcoVec<Height, C>>>,
}

impl<C: FiatType> FiatBlock<C> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let cents: EagerVec<PcoVec<Height, C>> =
            EagerVec::forced_import(db, &format!("{name}_cents"), version)?;
        let usd =
            LazyVecFrom1::transformed::<C::ToDollars>(name, version, cents.read_only_boxed_clone());
        Ok(Self { usd, cents })
    }
}
