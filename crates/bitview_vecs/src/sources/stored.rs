use brk_error::Result;
use brk_types::Version;
use vecdb::{
    BudgetedCachedVec, CacheBudget, Database, EagerVec, ImportableVec, PcoVec, PcoVecValue, Rw,
    StorageMode, VecIndex,
};

/// One stored source and its shared, budgeted cache.
pub type StoredSeries<I, T, M = Rw> =
    BudgetedCachedVec<<M as StorageMode>::Stored<EagerVec<PcoVec<I, T>>>>;

pub fn import_stored<I: VecIndex, T: PcoVecValue>(
    cache: &'static CacheBudget,
    db: &Database,
    name: &str,
    version: Version,
) -> Result<StoredSeries<I, T>> {
    Ok(cache.wrap(EagerVec::forced_import(db, name, version)?))
}
