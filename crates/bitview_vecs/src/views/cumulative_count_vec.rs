use std::convert::Infallible;

use brk_types::{Height, StoredU16, StoredU64};
use vecdb::{
    AnyVec, PrintableIndex, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, TypedVec, Version,
    short_type_name,
};

use super::prefix_sum_checkpoints::{INTERVAL as CHECKPOINT_INTERVAL, PrefixSumCheckpoints};

/// Cumulative `u64` counts over a range-readable `u16` block source.
/// Stores one prefix checkpoint per 256 blocks instead of a full cumulative history.
pub struct CumulativeCountVec {
    block: ReadableBoxedVec<Height, StoredU16>,
    checkpoints: PrefixSumCheckpoints,
}

impl CumulativeCountVec {
    pub fn new(block: &(impl ReadableCloneableVec<Height, StoredU16> + ?Sized)) -> Self {
        Self {
            block: block.read_only_boxed_clone(),
            checkpoints: PrefixSumCheckpoints::default(),
        }
    }

    fn cumulative_at(&self, index: usize) -> Option<StoredU64> {
        (index < self.block.len()).then(|| StoredU64::from(self.sum_before(index + 1)))
    }

    fn for_each_cumulative(&self, from: usize, to: usize, mut each: impl FnMut(StoredU64)) {
        self.try_fold_cumulative(from, to, (), |(), value| {
            each(value);
            Ok::<_, Infallible>(())
        })
        .unwrap();
    }

    fn try_fold_cumulative<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, StoredU64) -> Result<B, E>,
    ) -> Result<B, E> {
        let to = to.min(self.block.len());
        if from >= to {
            return Ok(init);
        }

        let mut cumulative = self.sum_before(from);
        self.block
            .try_fold_range_at(from, to, init, |accumulator, value| {
                cumulative += Self::as_u64(&value);
                fold(accumulator, StoredU64::from(cumulative))
            })
    }

    #[inline]
    fn sum_before(&self, end: usize) -> u64 {
        self.checkpoints.sum_before(&self.block, end, Self::as_u64)
    }

    /// Reuse the preceding sum only when advancing scans no more values than
    /// restarting from the nearest checkpoint. Large gaps remain bounded.
    fn advance_sum(&self, end: usize, state: &mut (usize, u64)) -> u64 {
        let (previous, sum) = *state;
        let value = if end >= previous && end - previous <= end % CHECKPOINT_INTERVAL {
            self.block
                .fold_range_at(previous, end, sum, |sum, value| sum + Self::as_u64(&value))
        } else {
            self.sum_before(end)
        };
        *state = (end, value);
        value
    }

    #[inline(always)]
    fn as_u64(value: &StoredU16) -> u64 {
        u64::from(**value)
    }
}

impl Clone for CumulativeCountVec {
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
            checkpoints: self.checkpoints.clone(),
        }
    }
}

impl AnyVec for CumulativeCountVec {
    fn version(&self) -> Version {
        self.block.version()
    }

    fn name(&self) -> &str {
        self.block.name()
    }

    fn len(&self) -> usize {
        self.block.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        <Height as PrintableIndex>::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<StoredU64>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<StoredU64>()
    }
}

impl TypedVec for CumulativeCountVec {
    type I = Height;
    type T = StoredU64;
}

impl ReadableVec<Height, StoredU64> for CumulativeCountVec {
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<StoredU64>) {
        buf.reserve(to.saturating_sub(from));
        self.for_each_cumulative(from, to, |value| buf.push(value));
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(StoredU64)) {
        self.for_each_cumulative(from, to, each);
    }

    fn fold_range_at<B, F: FnMut(B, StoredU64) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> B {
        self.try_fold_cumulative(from, to, init, |accumulator, value| {
            Ok::<_, Infallible>(fold(accumulator, value))
        })
        .unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, StoredU64) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        fold: F,
    ) -> Result<B, E> {
        self.try_fold_cumulative(from, to, init, fold)
    }

    fn collect_one_at(&self, index: usize) -> Option<StoredU64> {
        self.cumulative_at(index)
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<StoredU64>) {
        let len = self.block.len();
        let mut state = (0, 0);
        out.reserve(indices.len());
        indices
            .iter()
            .take_while(|&&index| index < len)
            .for_each(|&index| {
                out.push(StoredU64::from(self.advance_sum(index + 1, &mut state)));
            });
    }
}

#[cfg(test)]
mod tests {
    static TEST_CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);
    use brk_types::{Height, StoredU16, StoredU64, Version};
    use tempfile::tempdir;
    use vecdb::{
        AnyStoredVec, Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec,
        PcoVec, ReadOnlyClone, WritableVec,
    };

    use super::*;

    #[test]
    fn sorted_counts_reuse_tails_and_handle_gaps_duplicates_and_rewrites() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut block = EagerVec::<PcoVec<Height, StoredU16, Budgeted>>::forced_import_with(
            ImportOptions::new(&db, "sorted", Version::ONE).with_cache_budget(&TEST_CACHE),
        )
        .unwrap();
        for i in 0..4300 {
            block.push(StoredU16::new((i % 17) as u16));
        }
        block.write().unwrap();
        let cached = block.read_only_clone();
        let count = CumulativeCountVec::new(&cached);
        for rewrite in [false, true] {
            if rewrite {
                block.truncate_if_needed_at(4000).unwrap();
                for _ in 4000..4300 {
                    block.push(StoredU16::new(19));
                }
                block.write().unwrap();
            }
            let values = block.collect_range_at(0, 4300);
            let mut sum = 0u64;
            let cumulative: Vec<_> = values
                .iter()
                .map(|v| {
                    sum += u64::from(**v);
                    StoredU64::from(sum)
                })
                .collect();
            for indices in [
                vec![],
                vec![usize::MAX],
                vec![
                    0,
                    0,
                    254,
                    255,
                    255,
                    256,
                    257,
                    2047,
                    2048,
                    4299,
                    4300,
                    usize::MAX,
                ],
                (0..4300).collect(),
                (0..4300).step_by(13).flat_map(|i| [i, i]).collect(),
            ] {
                let mut actual = vec![StoredU64::from(99u64)];
                count.read_sorted_into_at(&indices, &mut actual);
                assert_eq!(
                    &actual[1..],
                    indices
                        .iter()
                        .filter_map(|&i| cumulative.get(i).copied())
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn reconstructs_cumulative_counts_after_rewrites() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut block: EagerVec<PcoVec<Height, StoredU16, Budgeted>> =
            EagerVec::forced_import_with(
                ImportOptions::new(&db, "count", Version::ONE).with_cache_budget(&TEST_CACHE),
            )
            .unwrap();

        let mut expected = Vec::new();
        let mut total = 0_u64;
        for index in 0..600 {
            let value = (index % 7) as u16;
            total += u64::from(value);
            expected.push(StoredU64::from(total));
            block.push(StoredU16::new(value));
        }
        block.write().unwrap();

        let count = CumulativeCountVec::new(&block);

        assert_eq!(count.cumulative_at(599), Some(expected[599]));

        let mut reconstructed = Vec::new();
        count.for_each_cumulative(250, 270, |value| reconstructed.push(value));
        assert_eq!(reconstructed, expected[250..270]);

        block.truncate_if_needed_at(0).unwrap();
        for _ in 0..600 {
            block.push(StoredU16::new(1));
        }
        block.write().unwrap();

        assert_eq!(count.cumulative_at(599), Some(StoredU64::from(600_u64)));
    }
}
