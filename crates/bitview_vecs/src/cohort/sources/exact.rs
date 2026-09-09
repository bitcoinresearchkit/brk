use std::ops::AddAssign;

use bitview_cohort::{Filter, UTXOOverlappingValues, UTXOValues};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, PcoVecValue, Rw, StorageMode};

use super::UTXOSources;
use crate::StoredSeries;

/// Cohorts whose overlapping results are supplied independently.
#[derive(Traversable)]
pub struct ExactUTXOSources<T: PcoVecValue, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub stored: UTXOSources<T, M>,
}

impl<T: PcoVecValue + AddAssign> ExactUTXOSources<T> {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
    ) -> Result<Self> {
        Ok(Self {
            stored: UTXOSources::forced_import(cache, db, name, version)?,
        })
    }
    pub fn get(&self, filter: &Filter) -> Option<&StoredSeries<Height, T>> {
        self.stored.get(filter)
    }
    pub fn push(&mut self, direct: UTXOValues<T>, overlapping: UTXOOverlappingValues<T>) {
        self.stored.push_exact(direct, overlapping);
    }
    pub fn min_len(&self) -> usize {
        self.stored.min_len()
    }
    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.stored.collect_vecs_mut()
    }
}
