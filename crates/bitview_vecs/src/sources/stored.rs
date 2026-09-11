use brk_error::Result;
use brk_types::Version;
use vecdb::{
    Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec, PcoVec, PcoVecValue,
    Rw, StorageMode, VecIndex,
};

/// One stored source and its shared, budgeted cache.
pub type StoredSeries<I, T, M = Rw> = <M as StorageMode>::Stored<EagerVec<PcoVec<I, T, Budgeted>>>;

pub fn import_stored<I: VecIndex, T: PcoVecValue>(
    cache: &'static CacheBudget,
    db: &Database,
    name: &str,
    version: Version,
) -> Result<StoredSeries<I, T>> {
    Ok(EagerVec::forced_import_with(
        ImportOptions::new(db, name, version).with_cache_budget(cache),
    )?)
}
