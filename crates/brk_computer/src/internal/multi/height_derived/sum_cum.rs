//! ComputedHeightDerivedSumCum - aggregates derived from an external height source.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, Database, Exit, GenericStoredVec, IterableBoxedVec, IterableCloneableVec,
    IterableVec, VecIndex,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedVecValue, CumulativeVec, LazyDateDerivedSumCum, LazySumCum, NumericValue, SumCum,
        compute_cumulative_extend,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedSumCum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "cumulative")]
    pub height_cumulative: CumulativeVec<Height, T>,
    pub dateindex: SumCum<DateIndex, T>,
    #[deref]
    #[deref_mut]
    pub dates: LazyDateDerivedSumCum<T>,
    pub difficultyepoch: LazySumCum<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedSumCum<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        height_source: IterableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height_cumulative = CumulativeVec::forced_import(db, name, v)?;
        let dateindex = SumCum::forced_import_sum_raw(db, name, v)?;

        let dates = LazyDateDerivedSumCum::from_sources(
            name,
            v,
            dateindex.boxed_sum(),
            dateindex.boxed_cumulative(),
            indexes,
        );

        let difficultyepoch = LazySumCum::from_sources_sum_raw(
            name,
            v,
            height_source.boxed_clone(),
            height_cumulative.boxed_clone(),
            indexes.difficultyepoch.identity.boxed_clone(),
        );

        Ok(Self {
            height_cumulative,
            dateindex,
            dates,
            difficultyepoch,
        })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_height_cumulative(starting_indexes.height, height_source, exit)?;
        self.compute_dateindex_sum_cum(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )
    }

    fn compute_height_cumulative(
        &mut self,
        max_from: Height,
        source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        compute_cumulative_extend(max_from, source, &mut self.height_cumulative.0, exit)
    }

    fn compute_dateindex_sum_cum(
        &mut self,
        starting_dateindex: DateIndex,
        height_source: &impl IterableVec<Height, T>,
        first_indexes: &impl IterableVec<DateIndex, Height>,
        count_indexes: &impl IterableVec<DateIndex, StoredU64>,
        exit: &Exit,
    ) -> Result<()> {
        let sum_vec = &mut self.dateindex.sum.0;
        let cumulative_vec = &mut self.dateindex.cumulative.0;

        let combined_version =
            height_source.version() + first_indexes.version() + count_indexes.version();
        sum_vec.validate_computed_version_or_reset(combined_version)?;
        cumulative_vec.validate_computed_version_or_reset(combined_version)?;

        let index = starting_dateindex
            .to_usize()
            .min(sum_vec.len())
            .min(cumulative_vec.len());

        let mut cumulative = if index > 0 {
            cumulative_vec.iter().get_unwrap((index - 1).into())
        } else {
            T::from(0_usize)
        };

        let mut source_iter = height_source.iter();
        let mut count_iter = count_indexes.iter().skip(index);

        first_indexes.iter().enumerate().skip(index).try_for_each(
            |(idx, first_height)| -> Result<()> {
                let count = *count_iter.next().unwrap() as usize;

                source_iter.set_position(first_height);
                let sum: T = (&mut source_iter)
                    .take(count)
                    .fold(T::from(0_usize), |acc, v| acc + v);

                cumulative += sum;
                sum_vec.truncate_push_at(idx, sum)?;
                cumulative_vec.truncate_push_at(idx, cumulative)?;

                Ok(())
            },
        )?;

        let _lock = exit.lock();
        sum_vec.write()?;
        cumulative_vec.write()?;

        Ok(())
    }
}
