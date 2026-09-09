use bitview_cohort::{AgeRange, CohortContext};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, BytesVecValue, Database, ImportableVec, Rw, StorageMode,
    WritableVec,
};

use super::UTXOTermSources;

/// Exact raw inputs for disjoint age bands and the stored holder aggregates.
#[derive(Deref, DerefMut, Traversable)]
pub struct UTXOAgeSources<T: BytesVecValue, M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub aggregate: UTXOTermSources<T, M>,
    pub age: AgeRange<M::Stored<BytesVec<Height, T>>>,
}

impl<T: BytesVecValue + Copy> UTXOAgeSources<T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            aggregate: UTXOTermSources::forced_import(db, name, version)?,
            age: AgeRange::try_new(|id| {
                BytesVec::forced_import(
                    db,
                    &CohortContext::Utxo.metric_name(id, name),
                    version + Version::ONE,
                )
            })?,
        })
    }

    pub fn push_age(&mut self, values: &AgeRange<T>) {
        for (target, &value) in self.age.iter_mut().zip(values.iter()) {
            target.push(value);
        }
    }

    pub fn len(&self) -> usize {
        self.age
            .iter()
            .map(AnyVec::len)
            .fold(self.aggregate.len(), usize::min)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.aggregate.collect_vecs_mut();
        vecs.extend(self.age.iter_mut().map(|v| v as &mut dyn AnyStoredVec));
        vecs
    }
}
