use brk_error::Result;
use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Database, Ro, Rw, StorageMode, VecIndex, Version};

use crate::internal::{ComputedVecValue, CumulativeVec, SumVec};

/// Sum + Cumulative (12% of usage)
#[derive(Traversable)]
pub struct SumCumulative<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub sum: SumVec<I, T, M>,
    #[traversable(flatten)]
    pub cumulative: CumulativeVec<I, T, M>,
}

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> SumCumulative<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self {
            sum: SumVec::forced_import(db, name, version)?,
            cumulative: CumulativeVec::forced_import(db, name, version)?,
        })
    }

    pub fn read_only_clone(&self) -> SumCumulative<I, T, Ro> {
        SumCumulative {
            sum: self.sum.read_only_clone(),
            cumulative: self.cumulative.read_only_clone(),
        }
    }
}
