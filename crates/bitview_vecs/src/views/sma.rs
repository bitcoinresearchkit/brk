use brk_types::{Cents, Height, StoredU64};
use vecdb::LazyDeltaVec;

use super::sma_average::SmaAverage;

/// Price SMA uses the same bounded current/lookback reads as other rolling deltas.
pub type LazySmaVec = LazyDeltaVec<Height, StoredU64, Cents, SmaAverage>;

#[cfg(test)]
mod tests {
    use crate::test_cache::init_cache;

    use std::{
        env, process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use brk_types::Version;
    use vecdb::{
        AnyStoredVec, Budgeted, Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec,
        ReadableVec, WritableVec,
    };

    use super::*;

    #[test]
    fn computes_rolling_average_from_one_shared_prefix_source() {
        init_cache();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("brk-lazy-sma-{}-{suffix}", process::id()));
        let db = Database::open(&path).unwrap();

        let mut prices: EagerVec<PcoVec<Height, Cents, Budgeted>> =
            EagerVec::forced_import(&db, "prices", Version::ONE).unwrap();
        let mut starts: EagerVec<PcoVec<Height, Height, Budgeted>> =
            EagerVec::forced_import(&db, "starts", Version::ONE).unwrap();

        for value in [100, 200, 300, 400] {
            prices.push(Cents::new(value));
        }
        for value in [0, 0, 1, 2] {
            starts.push(Height::new(value));
        }
        prices.write().unwrap();
        starts.write().unwrap();

        let mut prefix_sum: EagerVec<PcoVec<Height, StoredU64, Budgeted>> =
            EagerVec::forced_import(&db, "prefix", Version::ONE).unwrap();
        let mut sum = 0;
        for price in prices.collect() {
            sum += price.inner();
            prefix_sum.push(StoredU64::from(sum));
        }
        prefix_sum.write().unwrap();
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
