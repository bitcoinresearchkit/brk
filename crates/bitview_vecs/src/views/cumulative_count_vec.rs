use std::{convert::Infallible, sync::Arc};

use brk_types::{Height, StoredU16, StoredU64};
use parking_lot::RwLock;
use vecdb::{
    AnyVec, PrintableIndex, ReadableBoxedVec, ReadableVec, TypedVec, Version, short_type_name,
};

const CHECKPOINT_INTERVAL: usize = 256;

/// Cumulative `u64` counts over a shared `u16` block snapshot.
/// Stores one prefix checkpoint per 256 blocks instead of a full cumulative history.
pub struct CumulativeCountVec {
    block: ReadableBoxedVec<Height, StoredU16>,
    checkpoints: Arc<RwLock<Checkpoints>>,
}

struct Checkpoints {
    block: Arc<Vec<StoredU16>>,
    cumulative: Arc<Vec<u64>>,
}

impl CumulativeCountVec {
    pub fn new(block: impl ReadableVec<Height, StoredU16> + Clone + 'static) -> Self {
        Self {
            block: ReadableBoxedVec::new(block),
            checkpoints: Arc::new(RwLock::new(Checkpoints {
                block: Arc::new(Vec::new()),
                cumulative: Arc::new(vec![0]),
            })),
        }
    }

    fn cumulative_at(&self, index: usize) -> Option<StoredU64> {
        let (block, checkpoints) = self.snapshot();
        (index < block.len())
            .then(|| StoredU64::from(Self::sum_before(&block, &checkpoints, index + 1)))
    }

    fn for_each_cumulative(&self, from: usize, to: usize, mut each: impl FnMut(StoredU64)) {
        self.try_fold_cumulative(from, to, (), |(), value| {
            each(value);
            Ok::<_, Infallible>(())
        })
        .unwrap();
    }

    fn snapshot(&self) -> (Arc<Vec<StoredU16>>, Arc<Vec<u64>>) {
        let block = self.block.snapshot();

        {
            let checkpoints = self.checkpoints.read();
            if Arc::ptr_eq(&checkpoints.block, &block) {
                return (block, checkpoints.cumulative.clone());
            }
        }

        let cumulative = Self::build_checkpoints(&block);
        let mut checkpoints = self.checkpoints.write();
        if !Arc::ptr_eq(&checkpoints.block, &block) {
            checkpoints.block = block.clone();
            checkpoints.cumulative = cumulative;
        }

        (block, checkpoints.cumulative.clone())
    }

    fn try_fold_cumulative<B, E>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: impl FnMut(B, StoredU64) -> Result<B, E>,
    ) -> Result<B, E> {
        let (block, checkpoints) = self.snapshot();
        let to = to.min(block.len());
        if from >= to {
            return Ok(init);
        }

        let mut accumulator = init;
        let mut cumulative = Self::sum_before(&block, &checkpoints, from);

        for value in &block[from..to] {
            cumulative += Self::as_u64(value);
            accumulator = fold(accumulator, StoredU64::from(cumulative))?;
        }

        Ok(accumulator)
    }

    fn build_checkpoints(block: &[StoredU16]) -> Arc<Vec<u64>> {
        let mut checkpoints = Vec::with_capacity(block.len() / CHECKPOINT_INTERVAL + 1);
        let mut cumulative = 0;
        checkpoints.push(cumulative);

        for (index, value) in block.iter().enumerate() {
            cumulative += Self::as_u64(value);
            if (index + 1) % CHECKPOINT_INTERVAL == 0 {
                checkpoints.push(cumulative);
            }
        }

        Arc::new(checkpoints)
    }

    #[inline(always)]
    fn sum_before(block: &[StoredU16], checkpoints: &[u64], end: usize) -> u64 {
        let end = end.min(block.len());
        let checkpoint = end / CHECKPOINT_INTERVAL;
        let from = checkpoint * CHECKPOINT_INTERVAL;
        checkpoints[checkpoint] + block[from..end].iter().map(Self::as_u64).sum::<u64>()
    }

    /// Reuse the preceding sum only when advancing scans no more values than
    /// restarting from the nearest checkpoint. Large gaps remain bounded.
    fn advance_sum(
        block: &[StoredU16],
        checkpoints: &[u64],
        end: usize,
        state: &mut (usize, u64),
    ) -> u64 {
        let (previous, sum) = *state;
        let value = if end >= previous && end - previous <= end % CHECKPOINT_INTERVAL {
            sum + block[previous..end].iter().map(Self::as_u64).sum::<u64>()
        } else {
            Self::sum_before(block, checkpoints, end)
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
        let (block, checkpoints) = self.snapshot();
        let mut state = (0, 0);
        out.reserve(indices.len());
        indices
            .iter()
            .take_while(|&&index| index < block.len())
            .for_each(|&index| {
                out.push(StoredU64::from(Self::advance_sum(
                    &block,
                    &checkpoints,
                    index + 1,
                    &mut state,
                )));
            });
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{Height, StoredU16, StoredU64, Version};
    use vecdb::{AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, PcoVec, WritableVec};

    use super::*;
    use vecdb::{CachedReadableVec, ReadOnlyClone};

    #[test]
    fn sorted_counts_reuse_tails_and_handle_gaps_duplicates_and_rewrites() {
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut block =
            EagerVec::<PcoVec<Height, StoredU16>>::forced_import(&db, "sorted", Version::ONE)
                .unwrap();
        for i in 0..4300 {
            block.push(StoredU16::new((i % 17) as u16));
        }
        block.write().unwrap();
        let cached = CachedVec::wrap(block.read_only_clone());
        let count = CumulativeCountVec::new(cached.cached_boxed_clone());
        for rewrite in [false, true] {
            if rewrite {
                block.truncate_if_needed_at(4000).unwrap();
                for _ in 4000..4300 {
                    block.push(StoredU16::new(19));
                }
                block.write().unwrap();
                cached.invalidate();
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
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut block: EagerVec<PcoVec<Height, StoredU16>> =
            EagerVec::forced_import(&db, "count", Version::ONE).unwrap();

        let mut expected = Vec::new();
        let mut total = 0_u64;
        for index in 0..600 {
            let value = (index % 7) as u16;
            total += u64::from(value);
            expected.push(StoredU64::from(total));
            block.push(StoredU16::new(value));
        }
        block.write().unwrap();

        let mut block = CachedVec::wrap(block);
        let count = CumulativeCountVec::new(block.read_only_cached_boxed_clone());

        assert_eq!(count.cumulative_at(599), Some(expected[599]));

        let mut reconstructed = Vec::new();
        count.for_each_cumulative(250, 270, |value| reconstructed.push(value));
        assert_eq!(reconstructed, expected[250..270]);

        block.inner.truncate_if_needed_at(0).unwrap();
        for _ in 0..600 {
            block.inner.push(StoredU16::new(1));
        }
        block.inner.write().unwrap();
        block.invalidate();

        assert_eq!(count.cumulative_at(599), Some(StoredU64::from(600_u64)));
    }
}
