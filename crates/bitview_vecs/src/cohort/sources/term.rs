use bitview_cohort::{ByTerm, UTXOAggregate};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Height, Version};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, BytesVecValue, Database, ImportableVec, Rw, StorageMode,
    WritableVec,
};

#[derive(Traversable)]
pub struct UTXOTermSources<T: BytesVecValue, M: StorageMode = Rw> {
    pub term: ByTerm<M::Stored<BytesVec<Height, T>>>,
}

impl<T: BytesVecValue + Copy> UTXOTermSources<T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            term: ByTerm::try_new(|_, cohort| {
                BytesVec::forced_import(db, &format!("{cohort}_{name}"), version + Version::ONE)
            })?,
        })
    }
    pub fn push(&mut self, values: &UTXOAggregate<T>) {
        self.term.short.push(values.sth);
        self.term.long.push(values.lth);
    }
    pub fn len(&self) -> usize {
        self.term.short.len().min(self.term.long.len())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.term.short, &mut self.term.long]
    }
}
