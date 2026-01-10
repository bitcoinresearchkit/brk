//! ComputedHeightDerivedSum - dateindex storage + difficultyepoch + lazy time periods.

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
    internal::{ComputedVecValue, LazyDateDerivedSum, LazySum, NumericValue, SumVec},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedSum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: SumVec<DateIndex, T>,
    #[deref]
    #[deref_mut]
    pub dates: LazyDateDerivedSum<T>,
    pub difficultyepoch: LazySum<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedSum<T>
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
        let dateindex = SumVec::forced_import_raw(db, name, version + VERSION)?;
        let v = version + VERSION;

        Ok(Self {
            dates: LazyDateDerivedSum::from_source(name, v, dateindex.boxed_clone(), indexes),
            difficultyepoch: LazySum::from_source_raw(
                name,
                v,
                height_source,
                indexes.difficultyepoch.identity.boxed_clone(),
            ),
            dateindex,
        })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_from(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )
    }

    fn compute_from(
        &mut self,
        starting_dateindex: DateIndex,
        height_source: &impl IterableVec<Height, T>,
        first_indexes: &impl IterableVec<DateIndex, Height>,
        count_indexes: &impl IterableVec<DateIndex, StoredU64>,
        exit: &Exit,
    ) -> Result<()> {
        let sum_vec = &mut self.dateindex.0;

        let combined_version =
            height_source.version() + first_indexes.version() + count_indexes.version();
        sum_vec.validate_computed_version_or_reset(combined_version)?;

        let index = starting_dateindex.to_usize().min(sum_vec.len());

        let mut source_iter = height_source.iter();
        let mut count_iter = count_indexes.iter().skip(index);

        first_indexes.iter().enumerate().skip(index).try_for_each(
            |(idx, first_height)| -> Result<()> {
                let count = *count_iter.next().unwrap() as usize;

                source_iter.set_position(first_height);
                let sum: T = (&mut source_iter)
                    .take(count)
                    .fold(T::from(0_usize), |acc, v| acc + v);

                sum_vec.truncate_push_at(idx, sum)?;

                Ok(())
            },
        )?;

        let _lock = exit.lock();
        sum_vec.write()?;

        Ok(())
    }
}
