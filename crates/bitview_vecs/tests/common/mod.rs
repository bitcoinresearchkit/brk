use bitview_collections::PerResolution;
use bitview_vecs::{IndexSources, LazyPreviousDeltaVec};
use brk_types::Version;
use vecdb::{
    AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, PcoVecValue,
    ReadableCloneableVec, VecIndex, WritableVec,
};

pub fn stored<I: VecIndex, T: PcoVecValue>(
    db: &Database,
    name: &str,
    values: impl IntoIterator<Item = T>,
) -> EagerVec<PcoVec<I, T>> {
    let mut vec = EagerVec::forced_import(db, name, Version::ONE).unwrap();
    for value in values {
        vec.push(value);
    }
    vec.write().unwrap();
    vec
}

pub fn indexes(db: &Database) -> IndexSources {
    macro_rules! empty {
        ($name:expr) => {
            stored(db, $name, std::iter::empty())
        };
    }
    macro_rules! resolutions {
        ($prefix:literal, $method:ident) => {
            PerResolution {
                minute10: CachedVec::wrap(empty!(concat!($prefix, "_minute10"))).$method(),
                minute30: CachedVec::wrap(empty!(concat!($prefix, "_minute30"))).$method(),
                hour1: CachedVec::wrap(empty!(concat!($prefix, "_hour1"))).$method(),
                hour4: CachedVec::wrap(empty!(concat!($prefix, "_hour4"))).$method(),
                hour12: CachedVec::wrap(empty!(concat!($prefix, "_hour12"))).$method(),
                day1: CachedVec::wrap(empty!(concat!($prefix, "_day1"))).$method(),
                day3: CachedVec::wrap(empty!(concat!($prefix, "_day3"))).$method(),
                week1: CachedVec::wrap(empty!(concat!($prefix, "_week1"))).$method(),
                month1: CachedVec::wrap(empty!(concat!($prefix, "_month1"))).$method(),
                month3: CachedVec::wrap(empty!(concat!($prefix, "_month3"))).$method(),
                month6: CachedVec::wrap(empty!(concat!($prefix, "_month6"))).$method(),
                year1: CachedVec::wrap(empty!(concat!($prefix, "_year1"))).$method(),
                year10: CachedVec::wrap(empty!(concat!($prefix, "_year10"))).$method(),
                halving: CachedVec::wrap(empty!(concat!($prefix, "_halving"))).$method(),
                epoch: CachedVec::wrap(empty!(concat!($prefix, "_epoch"))).$method(),
            }
        };
    }
    IndexSources {
        first_height: resolutions!("first", read_only_boxed_clone),
        timestamp: resolutions!("timestamp", read_only_boxed_clone),
        height_minute10: empty!("height_minute10").read_only_boxed_clone(),
        height_day1: empty!("height_day1").read_only_boxed_clone(),
        height_tx_index_count: LazyPreviousDeltaVec::new(
            "tx_count",
            Version::ONE,
            empty!("tx_cumulative").read_only_boxed_clone(),
        ),
        day3_date: empty!("day3_date").read_only_boxed_clone(),
        week1_date: empty!("week1_date").read_only_boxed_clone(),
        month1_date: empty!("month1_date").read_only_boxed_clone(),
        month3_date: empty!("month3_date").read_only_boxed_clone(),
        month6_date: empty!("month6_date").read_only_boxed_clone(),
        year1_date: empty!("year1_date").read_only_boxed_clone(),
        year10_date: empty!("year10_date").read_only_boxed_clone(),
    }
}

#[allow(dead_code)]
pub static CACHE_BUDGET: vecdb::CacheBudget = vecdb::CacheBudget::new(2 * 1024 * 1024 * 1024);
