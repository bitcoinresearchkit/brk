use std::ops::AddAssign;

use bitview_cohort::UTXORows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use vecdb::{AnyStoredVec, Database, PcoVecValue, Rw, StorageMode};

use super::super::UTXOColumnarMetric;

#[derive(Traversable)]
pub struct CumulativeUTXOColumnarMetric<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[traversable(flatten)]
    pub matrices: UTXOColumnarMetric<T, M>,
    last: M::WriteOnly<super::state::CumulativeState<UTXORows<T>>>,
}

impl<T> CumulativeUTXOColumnarMetric<T>
where
    T: PcoVecValue + AddAssign + Copy + Default,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            matrices: UTXOColumnarMetric::forced_import(db, name, version)?,
            last: Default::default(),
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, rows: UTXORows<T>) {
        let len = self.matrices.min_len();
        let cumulative = self
            .last
            .accumulate(len, || self.matrices.collect_last(), rows);
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
