//! Generic consistency tests for all vec types.

use rawdb::Database;
use tempfile::TempDir;
use vecdb::{AnyStoredVec, EagerVec, ImportableVec, ReadableVec, StoredVec, Version, WritableVec};

/// Bulk and scalar reads agree after an initial write and repeated appends.
fn run_immediate_read_after_write<V>()
where
    V: StoredVec<I = usize, T = u64>,
{
    let temp_dir = TempDir::new().unwrap();
    let db = Database::open(&temp_dir.path().join("test.db")).unwrap();

    let mut vec: EagerVec<V> = EagerVec::forced_import(&db, "test_vec", Version::ONE).unwrap();

    let mut expected = Vec::new();
    for batch in 0..=10 {
        let start = expected.len();
        let end = start + if batch == 0 { 1000 } else { 100 };
        for i in start..end {
            let value = i as u64 * 100;
            vec.push(value);
            expected.push(value);
        }
        vec.flush().unwrap();

        for i in start..end {
            assert_eq!(vec.collect_one(i), Some(i as u64 * 100), "index {i}");
        }
        assert_eq!(vec.collect(), expected, "batch {batch}");
    }
}

// ============================================================================
// Test instantiation for BytesVec (no feature flag needed)
// ============================================================================

mod bytes {
    use super::*;
    use vecdb::BytesVec;
    type V = BytesVec<usize, u64>;

    #[test]
    fn immediate_read_after_write() {
        run_immediate_read_after_write::<V>();
    }
}

// ============================================================================
// Test instantiation for feature-gated vec types
// ============================================================================

#[cfg(feature = "zerocopy")]
mod zerocopy {
    use super::*;
    use vecdb::ZeroCopyVec;
    type V = ZeroCopyVec<usize, u64>;

    #[test]
    fn immediate_read_after_write() {
        run_immediate_read_after_write::<V>();
    }
}

#[cfg(feature = "pco")]
mod pco {
    use super::*;
    use vecdb::PcoVec;
    type V = PcoVec<usize, u64>;

    #[test]
    fn immediate_read_after_write() {
        run_immediate_read_after_write::<V>();
    }
}

#[cfg(feature = "lz4")]
mod lz4 {
    use super::*;
    use vecdb::LZ4Vec;
    type V = LZ4Vec<usize, u64>;

    #[test]
    fn immediate_read_after_write() {
        run_immediate_read_after_write::<V>();
    }
}

#[cfg(feature = "zstd")]
mod zstd {
    use super::*;
    use vecdb::ZstdVec;
    type V = ZstdVec<usize, u64>;

    #[test]
    fn immediate_read_after_write() {
        run_immediate_read_after_write::<V>();
    }
}
