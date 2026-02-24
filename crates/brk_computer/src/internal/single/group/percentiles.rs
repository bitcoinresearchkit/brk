use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, Ro, Rw, StorageMode, VecIndex, Version};

use crate::internal::{ComputedVecValue, MedianVec, Pct10Vec, Pct25Vec, Pct75Vec, Pct90Vec};

/// All percentiles (pct10, pct25, median, pct75, pct90)
#[derive(Traversable)]
pub struct Percentiles<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    pub pct10: Pct10Vec<I, T, M>,
    pub pct25: Pct25Vec<I, T, M>,
    pub median: MedianVec<I, T, M>,
    pub pct75: Pct75Vec<I, T, M>,
    pub pct90: Pct90Vec<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> Percentiles<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            pct10: Pct10Vec::forced_import(db, name, version)?,
            pct25: Pct25Vec::forced_import(db, name, version)?,
            median: MedianVec::forced_import(db, name, version)?,
            pct75: Pct75Vec::forced_import(db, name, version)?,
            pct90: Pct90Vec::forced_import(db, name, version)?,
        })
    }

    pub fn read_only_clone(&self) -> Percentiles<I, T, Ro> {
        Percentiles {
            pct10: self.pct10.read_only_clone(),
            pct25: self.pct25.read_only_clone(),
            median: self.median.read_only_clone(),
            pct75: self.pct75.read_only_clone(),
            pct90: self.pct90.read_only_clone(),
        }
    }
}
