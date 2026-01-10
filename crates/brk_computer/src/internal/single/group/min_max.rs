use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, VecIndex, Version};

use crate::internal::{ComputedVecValue, MaxVec, MinVec};

/// Min + Max
#[derive(Clone, Traversable)]
pub struct MinMax<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    #[traversable(flatten)]
    pub min: MinVec<I, T>,
    #[traversable(flatten)]
    pub max: MaxVec<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MinMax<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            min: MinVec::forced_import(db, name, version)?,
            max: MaxVec::forced_import(db, name, version)?,
        })
    }
}
