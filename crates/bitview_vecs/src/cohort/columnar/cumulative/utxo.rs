use std::ops::AddAssign;

use bitview_cohort::UTXORows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use vecdb::{AnyStoredVec, Database, PcoVecValue, Rw, StorageMode};

use super::super::UTXOColumns;

#[derive(Traversable)]
pub struct CumulativeUTXOColumns<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[traversable(flatten)]
    pub columns: UTXOColumns<T, M>,
    last: M::WriteOnly<super::super::state::CumulativeState<UTXORows<T>>>,
}

impl<T> CumulativeUTXOColumns<T>
where
    T: PcoVecValue + AddAssign + Copy + Default,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            columns: UTXOColumns::forced_import(db, name, version)?,
            last: Default::default(),
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, rows: UTXORows<T>) {
        let len = self.columns.min_len();
        let cumulative =
            self.last
                .accumulate(len, || self.columns.collect_last(), |row| *row += rows);
        self.columns.push(cumulative);
    }

    pub fn min_len(&self) -> usize {
        self.columns.min_len()
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.last = Default::default();
        self.columns.collect_vecs_mut()
    }
}
