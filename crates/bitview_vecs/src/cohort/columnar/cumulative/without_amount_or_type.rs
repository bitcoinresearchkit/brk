use std::ops::AddAssign;

use bitview_cohort::UTXOCoreRows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use vecdb::{AnyStoredVec, Database, PcoVecValue, Rw, StorageMode};

use super::super::UTXOColumnarMetricWithoutAmountOrType;

#[derive(Traversable)]
pub struct CumulativeUTXOColumnarMetricWithoutAmountOrType<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[traversable(flatten)]
    pub matrices: UTXOColumnarMetricWithoutAmountOrType<T, M>,
    last: M::WriteOnly<super::state::CumulativeState<UTXOCoreRows<T>>>,
}

impl<T> CumulativeUTXOColumnarMetricWithoutAmountOrType<T>
where
    T: PcoVecValue + AddAssign + Copy + Default,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            matrices: UTXOColumnarMetricWithoutAmountOrType::forced_import(db, name, version)?,
            last: Default::default(),
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, rows: impl Into<UTXOCoreRows<T>>) {
        let len = self.matrices.min_len();
        let cumulative = self
            .last
            .accumulate(len, || self.matrices.collect_last(), rows.into());
        self.matrices.push(cumulative);
    }

    pub fn min_len(&self) -> usize {
        self.matrices.min_len()
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.last = Default::default();
        self.matrices.collect_vecs_mut()
    }
}
