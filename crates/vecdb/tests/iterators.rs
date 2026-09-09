//! Generic iterator tests for all vec types.
//!
//! These tests run against any type implementing `StoredVec`, ensuring
//! consistent iterator behavior across BytesVec, ZeroCopyVec, PcoVec, LZ4Vec, and ZstdVec.

use rawdb::Database;
use tempfile::TempDir;
use vecdb::{Result, StoredVec, Version};

// ============================================================================
// Test Setup Helpers
// ============================================================================

fn setup_db() -> Result<(Database, TempDir)> {
    let temp = TempDir::new()?;
    let db = Database::open(temp.path())?;
    Ok((db, temp))
}

// ============================================================================
// Generic Clean Iterator Tests
// ============================================================================

mod clean_iter {
    use super::*;

    fn run_ranges<V>() -> Result<()>
    where
        V: StoredVec<I = usize, T = i32>,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;
        assert!(vec.collect().is_empty());
        assert_eq!(vec.collect_first(), None);
        assert_eq!(vec.collect_last(), None);

        for i in 0..10_000 {
            vec.push(i);
        }
        vec.write()?;
        let expected: Vec<i32> = (0..10_000).collect();
        assert_eq!(vec.collect(), expected);
        assert_eq!(vec.collect_first(), Some(0));
        assert_eq!(vec.collect_last(), Some(9_999));
        for i in [100, 500, 50] {
            assert_eq!(vec.collect_one(i), Some(i as i32));
        }
        for (from, to) in [
            (0, 0),
            (0, 25),
            (50, 52),
            (150, 250),
            (1, 10_000),
            (9_999, 10_000),
            (10_000, 10_000),
        ] {
            assert_eq!(vec.collect_range(from, to), expected[from..to]);
        }
        Ok(())
    }

    // ============================================================================
    // Test instantiation for each vec type
    // ============================================================================

    mod bytes {
        use vecdb::BytesVec;

        use super::*;

        type V = BytesVec<usize, i32>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    #[cfg(feature = "zerocopy")]
    mod zerocopy {
        use vecdb::ZeroCopyVec;

        use super::*;

        type V = ZeroCopyVec<usize, i32>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    #[cfg(feature = "pco")]
    mod pco {
        use vecdb::PcoVec;

        use super::*;

        type V = PcoVec<usize, i32>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    #[cfg(feature = "lz4")]
    mod lz4 {
        use vecdb::LZ4Vec;

        use super::*;

        type V = LZ4Vec<usize, i32>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    #[cfg(feature = "zstd")]
    mod zstd {
        use vecdb::ZstdVec;

        use super::*;

        type V = ZstdVec<usize, i32>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    // ============================================================================
    // EagerVec Tests (wrapping different underlying vec types)
    // ============================================================================

    #[cfg(feature = "zerocopy")]
    mod eager_zerocopy {
        use vecdb::{EagerVec, ZeroCopyVec};

        use super::*;

        type V = EagerVec<ZeroCopyVec<usize, i32>>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }

    #[cfg(feature = "pco")]
    mod eager_pco {
        use vecdb::{EagerVec, PcoVec};

        use super::*;

        type V = EagerVec<PcoVec<usize, i32>>;

        #[test]
        fn ranges() -> Result<()> {
            run_ranges::<V>()
        }
    }
}

// ============================================================================
// Generic Dirty Iterator Tests (stored + pushed data)
// ============================================================================

mod dirty_iter {
    use super::*;

    fn run_stored_and_pushed<V>() -> Result<()>
    where
        V: StoredVec<I = usize, T = i32>,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;
        for (stored, pushed) in [(50, 50), (8_000, 4_000), (10_000, 100)] {
            vec.reset()?;
            for i in 0..stored {
                vec.push(i);
            }
            let prefix: Vec<i32> = (0..stored).collect();
            assert_eq!(vec.collect(), prefix);
            vec.write()?;
            assert_eq!(vec.collect(), prefix);
            assert_eq!(vec.collect_last(), Some(stored - 1));
            let end = stored + pushed;
            for i in stored..end {
                vec.push(i);
            }
            let expected: Vec<i32> = (0..end).collect();
            assert_eq!(vec.len(), end as usize);
            assert_eq!(vec.collect(), expected);
            assert_eq!(vec.collect_last(), Some(end - 1));
            for (from, to) in [
                (0, end),
                (1, end),
                (stored - 1, stored + 1),
                (stored - 10, stored + 10),
                (stored, end),
                (end, end),
            ] {
                assert_eq!(
                    vec.collect_range(from as usize, to as usize),
                    expected[from as usize..to as usize]
                );
            }
        }
        Ok(())
    }

    // ============================================================================
    // Test instantiation for each vec type
    // ============================================================================

    mod bytes {
        use vecdb::BytesVec;

        use super::*;

        type V = BytesVec<usize, i32>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    #[cfg(feature = "zerocopy")]
    mod zerocopy {
        use vecdb::ZeroCopyVec;

        use super::*;

        type V = ZeroCopyVec<usize, i32>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    #[cfg(feature = "pco")]
    mod pco {
        use vecdb::PcoVec;

        use super::*;

        type V = PcoVec<usize, i32>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    #[cfg(feature = "lz4")]
    mod lz4 {
        use vecdb::LZ4Vec;

        use super::*;

        type V = LZ4Vec<usize, i32>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    #[cfg(feature = "zstd")]
    mod zstd {
        use vecdb::ZstdVec;

        use super::*;

        type V = ZstdVec<usize, i32>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    // ============================================================================
    // EagerVec Tests (wrapping different underlying vec types)
    // ============================================================================

    #[cfg(feature = "zerocopy")]
    mod eager_zerocopy {
        use vecdb::{EagerVec, ZeroCopyVec};

        use super::*;

        type V = EagerVec<ZeroCopyVec<usize, i32>>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }

    #[cfg(feature = "pco")]
    mod eager_pco {
        use vecdb::{EagerVec, PcoVec};

        use super::*;

        type V = EagerVec<PcoVec<usize, i32>>;

        #[test]
        fn stored_and_pushed() -> Result<()> {
            run_stored_and_pushed::<V>()
        }
    }
}

// ============================================================================
// Mutable Raw-Vector Tests (holes and updates)
// ============================================================================

mod raw_features {
    use vecdb::{BytesVec, MutableVec};

    use super::*;

    #[cfg(feature = "zerocopy")]
    use vecdb::ZeroCopyVec;

    // Generic test functions for MutableVec over raw vecs

    fn run_iter_skips_holes<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..10 {
            vec.push(i);
        }
        vec.write()?;

        // Delete some values (create holes)
        vec.delete_at(3);
        vec.delete_at(5);
        vec.delete_at(7);

        let collected: Vec<i32> = vec.collect();
        // Should skip holes: 0,1,2,4,6,8,9
        assert_eq!(collected, vec![0, 1, 2, 4, 6, 8, 9]);
        Ok(())
    }

    fn run_iter_with_updates<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..10 {
            vec.push(i);
        }
        vec.write()?;

        // Update some values
        vec.update_at(2, 200)?;
        vec.update_at(5, 500)?;
        vec.update_at(8, 800)?;

        let collected: Vec<i32> = vec.collect();
        assert_eq!(collected, vec![0, 1, 200, 3, 4, 500, 6, 7, 800, 9]);
        Ok(())
    }

    fn run_iter_with_holes_and_updates<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..10 {
            vec.push(i);
        }
        vec.write()?;

        // Create holes and updates
        vec.delete_at(1);
        vec.delete_at(3);
        vec.update_at(2, 200)?;
        vec.update_at(5, 500)?;

        let collected: Vec<i32> = vec.collect();
        // Should be: 0, (skip 1), 200, (skip 3), 4, 500, 6, 7, 8, 9
        assert_eq!(collected, vec![0, 200, 4, 500, 6, 7, 8, 9]);
        Ok(())
    }

    fn run_iter_holes_and_pushed<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..5 {
            vec.push(i);
        }
        vec.write()?;

        // Create holes in stored data
        vec.delete_at(1);
        vec.delete_at(3);

        // Push more data
        for i in 5..10 {
            vec.push(i);
        }

        let collected: Vec<i32> = vec.collect();
        // Should be: 0, (skip 1), 2, (skip 3), 4, 5, 6, 7, 8, 9
        assert_eq!(collected, vec![0, 2, 4, 5, 6, 7, 8, 9]);
        Ok(())
    }

    fn run_iter_updates_and_pushed<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..5 {
            vec.push(i);
        }
        vec.write()?;

        // Update some stored values
        vec.update_at(1, 100)?;
        vec.update_at(3, 300)?;

        // Push more data
        for i in 5..10 {
            vec.push(i);
        }

        let collected: Vec<i32> = vec.collect();
        assert_eq!(collected, vec![0, 100, 2, 300, 4, 5, 6, 7, 8, 9]);
        Ok(())
    }

    fn run_iter_skip_over_holes<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..20 {
            vec.push(i);
        }
        vec.write()?;

        // Create holes at indices 5, 6, 7
        vec.delete_at(5);
        vec.delete_at(6);
        vec.delete_at(7);

        // Skip past the holes — collect skips holes automatically
        let collected: Vec<i32> = vec.collect();
        // Should be: 0, 1, 2, 3, 4, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
        // (holes at 5,6,7 are skipped)
        assert_eq!(collected[5..10], [8, 9, 10, 11, 12]);
        Ok(())
    }

    fn run_fill_holes<V>() -> Result<()>
    where
        V: RawVecOps,
    {
        let (db, _temp) = setup_db()?;
        let mut vec = V::forced_import(&db, "test", Version::ONE)?;

        for i in 0..10 {
            vec.push(i);
        }
        vec.write()?;

        // Create holes
        vec.delete_at(2);
        vec.delete_at(5);

        // Fill first hole
        let idx = vec.fill_first_hole_or_push(999)?;
        assert_eq!(idx, 2);

        let collected: Vec<i32> = vec.collect();
        // 0,1,999,3,4,(skip 5),6,7,8,9
        assert_eq!(collected, vec![0, 1, 999, 3, 4, 6, 7, 8, 9]);
        Ok(())
    }

    // Helper trait for mutable raw-vector operations
    pub trait RawVecOps: StoredVec<I = usize, T = i32> {
        fn delete_at(&mut self, index: usize);
        fn update_at(&mut self, index: usize, value: i32) -> Result<()>;
        fn fill_first_hole_or_push(&mut self, value: i32) -> Result<usize>;
    }

    impl RawVecOps for MutableVec<BytesVec<usize, i32>> {
        fn delete_at(&mut self, index: usize) {
            MutableVec::<BytesVec<usize, i32>>::delete_at(self, index)
        }
        fn update_at(&mut self, index: usize, value: i32) -> Result<()> {
            MutableVec::<BytesVec<usize, i32>>::update_at(self, index, value)
        }
        fn fill_first_hole_or_push(&mut self, value: i32) -> Result<usize> {
            MutableVec::<BytesVec<usize, i32>>::fill_first_hole_or_push(self, value)
        }
    }

    #[cfg(feature = "zerocopy")]
    impl RawVecOps for MutableVec<ZeroCopyVec<usize, i32>> {
        fn delete_at(&mut self, index: usize) {
            MutableVec::<ZeroCopyVec<usize, i32>>::delete_at(self, index)
        }
        fn update_at(&mut self, index: usize, value: i32) -> Result<()> {
            MutableVec::<ZeroCopyVec<usize, i32>>::update_at(self, index, value)
        }
        fn fill_first_hole_or_push(&mut self, value: i32) -> Result<usize> {
            MutableVec::<ZeroCopyVec<usize, i32>>::fill_first_hole_or_push(self, value)
        }
    }

    // ============================================================================
    // BytesVec Tests
    // ============================================================================

    mod bytes {
        use super::*;

        #[test]
        fn iter_skips_holes() -> Result<()> {
            run_iter_skips_holes::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn iter_with_updates() -> Result<()> {
            run_iter_with_updates::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn iter_with_holes_and_updates() -> Result<()> {
            run_iter_with_holes_and_updates::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn iter_holes_and_pushed() -> Result<()> {
            run_iter_holes_and_pushed::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn iter_updates_and_pushed() -> Result<()> {
            run_iter_updates_and_pushed::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn iter_skip_over_holes() -> Result<()> {
            run_iter_skip_over_holes::<MutableVec<BytesVec<usize, i32>>>()
        }
        #[test]
        fn fill_holes() -> Result<()> {
            run_fill_holes::<MutableVec<BytesVec<usize, i32>>>()
        }
    }

    // ============================================================================
    // ZeroCopyVec Tests
    // ============================================================================

    #[cfg(feature = "zerocopy")]
    mod zerocopy {
        use super::*;

        #[test]
        fn iter_skips_holes() -> Result<()> {
            run_iter_skips_holes::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn iter_with_updates() -> Result<()> {
            run_iter_with_updates::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn iter_with_holes_and_updates() -> Result<()> {
            run_iter_with_holes_and_updates::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn iter_holes_and_pushed() -> Result<()> {
            run_iter_holes_and_pushed::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn iter_updates_and_pushed() -> Result<()> {
            run_iter_updates_and_pushed::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn iter_skip_over_holes() -> Result<()> {
            run_iter_skip_over_holes::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
        #[test]
        fn fill_holes() -> Result<()> {
            run_fill_holes::<MutableVec<ZeroCopyVec<usize, i32>>>()
        }
    }
}
