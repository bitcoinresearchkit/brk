use std::ops::AddAssign;

use bitview_cohort::UTXOCoreRows;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::Version;
use vecdb::{AnyStoredVec, CacheBudget, Database, PcoVecValue, Rw, StorageMode};

use super::super::{UTXOCoreColumns, state::CumulativeState};

#[derive(Traversable)]
pub struct CumulativeUTXOCoreColumns<T, M: StorageMode = Rw>
where
    T: PcoVecValue,
{
    #[traversable(flatten)]
    pub columns: UTXOCoreColumns<T, M>,
    last: M::WriteOnly<CumulativeState<UTXOCoreRows<T>>>,
}

impl<T> CumulativeUTXOCoreColumns<T>
where
    T: PcoVecValue + AddAssign + Copy + Default,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            columns: UTXOCoreColumns::forced_import(cache, db, name, version)?,
            last: Default::default(),
        })
    }

    #[inline(always)]
    pub fn push_block(&mut self, rows: impl Into<UTXOCoreRows<T>>) {
        let len = self.columns.min_len();
        let cumulative = self.last.accumulate(
            len,
            || self.columns.collect_last(),
            |row| *row += rows.into(),
        );
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
