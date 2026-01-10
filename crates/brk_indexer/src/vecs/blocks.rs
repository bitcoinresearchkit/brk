use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BlockHash, Height, StoredF64, StoredU64, Timestamp, Version, Weight};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

use crate::parallel_import;

#[derive(Clone, Traversable)]
pub struct BlocksVecs {
    pub blockhash: BytesVec<Height, BlockHash>,
    #[traversable(wrap = "difficulty", rename = "raw")]
    pub difficulty: PcoVec<Height, StoredF64>,
    /// Doesn't guarantee continuity due to possible reorgs and more generally the nature of mining
    #[traversable(wrap = "time")]
    pub timestamp: PcoVec<Height, Timestamp>,
    pub total_size: PcoVec<Height, StoredU64>,
    pub weight: PcoVec<Height, Weight>,
}

impl BlocksVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        let (blockhash, difficulty, timestamp, total_size, weight) = parallel_import! {
            blockhash = BytesVec::forced_import(db, "blockhash", version),
            difficulty = PcoVec::forced_import(db, "difficulty", version),
            timestamp = PcoVec::forced_import(db, "timestamp", version),
            total_size = PcoVec::forced_import(db, "total_size", version),
            weight = PcoVec::forced_import(db, "weight", version),
        };
        Ok(Self {
            blockhash,
            difficulty,
            timestamp,
            total_size,
            weight,
        })
    }

    pub fn truncate(&mut self, height: Height, stamp: Stamp) -> Result<()> {
        self.blockhash
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.difficulty
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.timestamp
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.total_size
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.weight
            .truncate_if_needed_with_stamp(height, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.blockhash as &mut dyn AnyStoredVec,
            &mut self.difficulty,
            &mut self.timestamp,
            &mut self.total_size,
            &mut self.weight,
        ]
        .into_par_iter()
    }
}
