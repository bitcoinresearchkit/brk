use std::ops::AddAssign;

use bitview_cohort::{Amount, AmountRange, AmountRangeId, CohortContext};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, CacheBudget, Database, PcoVecValue, ReadableCloneableVec, ReadableColumnarVec,
    ReadableVec, Rw, StorageMode, WritableVec,
};

use super::state::CumulativeState;
use crate::ColumnarPerBlock;

/// Exact amount columns and their range/under/over views.
#[derive(Deref, DerefMut, Traversable)]
pub struct ColumnarAmount<T: PcoVecValue, S: Clone, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub per_block: ColumnarPerBlock<T, AmountRangeId, Amount<S>, M>,
    last: M::WriteOnly<CumulativeState<AmountRange<T>>>,
}

impl<T: PcoVecValue + AddAssign, S: Clone> ColumnarAmount<T, S> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        storage_name: &str,
        context: CohortContext,
        metric: &str,
        version: Version,
        mut build: impl FnMut(&str, &dyn ReadableCloneableVec<Height, T>) -> S,
    ) -> Result<Self> {
        let per_block =
            ColumnarPerBlock::forced_import(cache, db, storage_name, version, |source| {
                Amount::new(|filter, cohort_name| {
                    let name = context.metric_name(&filter, cohort_name, metric);
                    match AmountRangeId::matching(&filter) {
                        Some(column) => build(&name, &source.column(&name, version, column)),
                        None => build(
                            &name,
                            &source.sum_columns(
                                &name,
                                version,
                                AmountRangeId::included_by(&filter),
                            ),
                        ),
                    }
                })
            })?;
        Ok(Self {
            per_block,
            last: Default::default(),
        })
    }

    #[inline(always)]
    pub fn push_cumulative(&mut self, delta: &AmountRange<T>)
    where
        T: Default,
    {
        let len = self.per_block.len();
        let row = self.last.accumulate(
            len,
            || self.per_block.height.collect_last(),
            |row| {
                for (value, &delta) in row.iter_mut().zip(delta.iter()) {
                    *value += delta;
                }
            },
        );
        self.per_block.push(row);
    }

    pub fn reset(&mut self) -> Result<()> {
        self.last = Default::default();
        self.per_block.height.reset().map_err(Into::into)
    }

    pub fn stored_mut(&mut self) -> &mut dyn AnyStoredVec {
        self.last = Default::default();
        self.per_block.stored_mut()
    }
}
