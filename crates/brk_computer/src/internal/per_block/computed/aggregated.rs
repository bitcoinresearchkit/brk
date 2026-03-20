use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Height;
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, ReadableVec, Rw, StorageMode, VecIndex, VecValue,
    Version, WritableVec,
};

use crate::{
    indexes,
    internal::{CachedWindowStarts, NumericValue, PerBlock, RollingComplete, WindowStarts},
};

#[derive(Traversable)]
pub struct PerBlockAggregated<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub sum: PerBlock<T, M>,
    pub cumulative: PerBlock<T, M>,
    pub rolling: RollingComplete<T, M>,
}

impl<T> PerBlockAggregated<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let sum = PerBlock::forced_import(db, &format!("{name}_sum"), version, indexes)?;
        let cumulative =
            PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let rolling = RollingComplete::forced_import(
            db,
            name,
            version,
            indexes,
            &cumulative.height,
            cached_starts,
        )?;

        Ok(Self {
            sum,
            cumulative,
            rolling,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute<A>(
        &mut self,
        max_from: Height,
        source: &impl ReadableVec<A, T>,
        first_indexes: &impl ReadableVec<Height, A>,
        count_indexes: &impl ReadableVec<Height, brk_types::StoredU64>,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
        A: VecIndex + VecValue + brk_types::CheckedSub<A>,
    {
        let combined_version = source.version() + first_indexes.version() + count_indexes.version();

        let mut index = max_from;
        index = {
            self.sum
                .height
                .validate_computed_version_or_reset(combined_version)?;
            index.min(Height::from(self.sum.height.len()))
        };
        index = {
            self.cumulative
                .height
                .validate_computed_version_or_reset(combined_version)?;
            index.min(Height::from(self.cumulative.height.len()))
        };

        let start = index.to_usize();

        self.sum.height.truncate_if_needed_at(start)?;
        self.cumulative.height.truncate_if_needed_at(start)?;

        let mut cumulative_val = index.decremented().map_or(T::from(0_usize), |idx| {
            self.cumulative
                .height
                .collect_one_at(idx.to_usize())
                .unwrap_or(T::from(0_usize))
        });

        let fi_len = first_indexes.len();
        let first_indexes_batch: Vec<A> = first_indexes.collect_range_at(start, fi_len);
        let count_indexes_batch: Vec<brk_types::StoredU64> =
            count_indexes.collect_range_at(start, fi_len);

        first_indexes_batch
            .into_iter()
            .zip(count_indexes_batch)
            .try_for_each(|(first_index, count_index)| -> Result<()> {
                let count = u64::from(count_index) as usize;
                let effective_count = count.saturating_sub(skip_count);
                let effective_first_index = first_index + skip_count.min(count);

                let efi = effective_first_index.to_usize();
                let sum_val = source.fold_range_at(
                    efi,
                    efi + effective_count,
                    T::from(0_usize),
                    |acc, val| acc + val,
                );

                self.sum.height.push(sum_val);
                cumulative_val += sum_val;
                self.cumulative.height.push(cumulative_val);

                Ok(())
            })?;

        let _lock = exit.lock();
        self.sum.height.write()?;
        self.cumulative.height.write()?;
        drop(_lock);

        self.rolling
            .compute(max_from, windows, &self.sum.height, exit)?;
        Ok(())
    }
}
