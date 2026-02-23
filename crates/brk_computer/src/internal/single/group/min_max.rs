use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, Ro, Rw, StorageMode, VecIndex, Version};

use crate::internal::{ComputedVecValue, MaxVec, MinVec};

/// Min + Max
#[derive(Traversable)]
pub struct MinMax<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub min: MinVec<I, T, M>,
    #[traversable(flatten)]
    pub max: MaxVec<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MinMax<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            min: MinVec::forced_import(db, name, version)?,
            max: MaxVec::forced_import(db, name, version)?,
        })
    }

    pub fn read_only_clone(&self) -> MinMax<I, T, Ro> {
        MinMax {
            min: self.min.read_only_clone(),
            max: self.max.read_only_clone(),
        }
    }
}
