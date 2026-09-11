use brk_types::{Cents, Height, StoredU64};
use vecdb::LazyDeltaVec;

use super::sma_average::SmaAverage;

/// Price SMA uses the same bounded current/lookback reads as other rolling deltas.
pub type LazySmaVec = LazyDeltaVec<Height, StoredU64, Cents, SmaAverage>;

#[cfg(test)]
mod tests {
    use std::{
        env, process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use brk_types::Version;
    use vecdb::{
        AnyStoredVec, Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec,
        PcoVec, ReadableCloneableVec, ReadableVec, WritableVec,
    };

    use super::*;
    use crate::SmaPrefixSumVec;

    static TEST_CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);

    #[test]
    fn computes_rolling_average_from_one_shared_prefix_source() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("brk-lazy-sma-{}-{suffix}", process::id()));
        let db = Database::open(&path).unwrap();

        let mut prices: EagerVec<PcoVec<Height, Cents, Budgeted>> = EagerVec::forced_import_with(
            ImportOptions::new(&db, "prices", Version::ONE).with_cache_budget(&TEST_CACHE),
        )
        .unwrap();
        let mut starts: EagerVec<PcoVec<Height, Height, Budgeted>> = EagerVec::forced_import_with(
            ImportOptions::new(&db, "starts", Version::ONE).with_cache_budget(&TEST_CACHE),
        )
        .unwrap();

        for value in [100, 200, 300, 400] {
            prices.push(Cents::new(value));
        }
        for value in [0, 0, 1, 2] {
            starts.push(Height::new(value));
        }
        prices.write().unwrap();
        starts.write().unwrap();

        let prefix_sum = SmaPrefixSumVec::new("prefix", Version::ONE, &prices);
        let sma = LazySmaVec::new(
            "sma",
            Version::ONE,
            prefix_sum.read_only_boxed_clone(),
            starts.read_only_boxed_clone(),
        );

        assert_eq!(
            prefix_sum.collect().as_slice(),
            [
                StoredU64::new(100),
                StoredU64::new(300),
                StoredU64::new(600),
                StoredU64::new(1_000),
            ],
        );
        assert_eq!(
            sma.collect_range(Height::ZERO, Height::new(4)),
            [
                Cents::new(100),
                Cents::new(150),
                Cents::new(250),
                Cents::new(350),
            ],
        );
    }
}
