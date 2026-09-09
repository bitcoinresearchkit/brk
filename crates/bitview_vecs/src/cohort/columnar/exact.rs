use std::ops::AddAssign;

use bitview_cohort::{Filter, UTXOAggregateRows, UTXORows};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, PcoVecValue, ReadableBoxedVec, Rw, StorageMode};

use super::{UTXOColumns, UTXOOverlappingColumns};

/// Direct UTXO columns plus independently calculated overlapping cohorts.
/// Use when cohort results cannot be obtained by summing the direct columns.
#[derive(Traversable)]
pub struct ExactUTXOColumns<T: PcoVecValue, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub direct: UTXOColumns<T, M>,
    pub overlapping: UTXOOverlappingColumns<T, M>,
}

impl<T: PcoVecValue + AddAssign> ExactUTXOColumns<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            direct: UTXOColumns::forced_import(cache, db, name, version)?,
            overlapping: UTXOOverlappingColumns::forced_import(cache, db, name, version)?,
        })
    }

    pub fn source(
        &self,
        filter: &Filter,
        name: &str,
        version: Version,
    ) -> Option<ReadableBoxedVec<Height, T>> {
        self.direct
            .direct_source(filter, name, version)
            .or_else(|| self.overlapping.source(filter, name, version))
    }

    #[inline(always)]
    pub fn push(&mut self, direct: UTXORows<T>, overlapping: UTXOAggregateRows<T>) {
        self.direct.push(direct);
        self.overlapping.push(overlapping);
    }

    pub fn min_len(&self) -> usize {
        self.direct.min_len().min(self.overlapping.min_len())
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.direct.collect_vecs_mut();
        vecs.extend(self.overlapping.collect_vecs_mut());
        vecs
    }
}
