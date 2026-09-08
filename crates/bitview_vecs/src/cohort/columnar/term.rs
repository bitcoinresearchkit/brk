use brk_error::Result;

use bitview_cohort::{ByTerm, TermId, UTXOAggregate};
use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, BytesVecValue, ColumnarVec, Database, ImportableVec, Rw,
    StorageMode, WritableVec,
};

/// Short- and long-term-holder source columns for byte-stored values.
#[derive(Traversable)]
pub struct UTXOTermColumns<T, M: StorageMode = Rw>
where
    T: BytesVecValue,
{
    pub height: M::Stored<ColumnarVec<BytesVec<Height, T>, TermId>>,
}

impl<T> UTXOTermColumns<T>
where
    T: BytesVecValue + Copy,
{
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            height: ImportableVec::forced_import(db, &format!("{name}_by_term"), version)?,
        })
    }

    #[inline(always)]
    pub fn push(&mut self, row: &UTXOAggregate<T>) {
        self.height.push(ByTerm {
            short: row.sth,
            long: row.lth,
        });
    }

    pub fn len(&self) -> usize {
        self.height.len()
    }

    pub fn is_empty(&self) -> bool {
        self.height.is_empty()
    }

    pub fn stored_mut(&mut self) -> &mut dyn AnyStoredVec {
        &mut self.height
    }
}
