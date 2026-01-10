use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, VecIndex, Version};

use crate::internal::{ComputedVecValue, MedianVec, Pct10Vec, Pct25Vec, Pct75Vec, Pct90Vec};

/// All percentiles (pct10, pct25, median, pct75, pct90)
#[derive(Clone, Traversable)]
pub struct Percentiles<I: VecIndex, T: ComputedVecValue + JsonSchema> {
    pub pct10: Pct10Vec<I, T>,
    pub pct25: Pct25Vec<I, T>,
    pub median: MedianVec<I, T>,
    pub pct75: Pct75Vec<I, T>,
    pub pct90: Pct90Vec<I, T>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Percentiles<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            pct10: Pct10Vec::forced_import(db, name, version)?,
            pct25: Pct25Vec::forced_import(db, name, version)?,
            median: MedianVec::forced_import(db, name, version)?,
            pct75: Pct75Vec::forced_import(db, name, version)?,
            pct90: Pct90Vec::forced_import(db, name, version)?,
        })
    }
}
