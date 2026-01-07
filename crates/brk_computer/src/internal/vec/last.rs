use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CheckedSub, StoredU64};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableVec, PcoVec, VecIndex, VecValue, Version};

use crate::internal::ComputedVecValue;

/// Last value in an aggregation period
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct LastVec<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    pub EagerVec<PcoVec<I, T>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> LastVec<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(db, name, version)?))
    }

    #[inline]
    pub fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    /// Compute last values from a source vec.
    ///
    /// For each output index I, takes the last value from the corresponding
    /// range in the source vec (indexed by A).
    pub fn compute_last<A>(
        &mut self,
        max_from: I,
        source: &impl IterableVec<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + CheckedSub<A>,
    {
        self.0.validate_computed_version_or_reset(
            source.version() + first_indexes.version() + count_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.0.len()));

        let mut source_iter = source.iter();
        let mut count_indexes_iter = count_indexes.iter().skip(index.to_usize());

        first_indexes
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, first_index)| -> Result<()> {
                let count_index = count_indexes_iter.next().unwrap();
                let count = *count_index as usize;

                if count == 0 {
                    panic!("should not compute last if count can be 0");
                }

                let last_index = first_index + (count - 1);
                let v = source_iter.get_unwrap(last_index);
                self.0.truncate_push_at(i, v)?;

                Ok(())
            })?;

        let _lock = exit.lock();
        self.0.write()?;

        Ok(())
    }
}
