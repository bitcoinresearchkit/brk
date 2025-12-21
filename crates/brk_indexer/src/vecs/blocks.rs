use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BlockHash, Height, StoredF64, StoredU64, Timestamp, Version, Weight};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportableVec, PcoVec, Stamp};

#[derive(Clone, Traversable)]
pub struct BlockVecs {
    pub height_to_blockhash: BytesVec<Height, BlockHash>,
    pub height_to_difficulty: PcoVec<Height, StoredF64>,
    /// Doesn't guarantee continuity due to possible reorgs and more generally the nature of mining
    pub height_to_timestamp: PcoVec<Height, Timestamp>,
    pub height_to_total_size: PcoVec<Height, StoredU64>,
    pub height_to_weight: PcoVec<Height, Weight>,
}

impl BlockVecs {
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            height_to_blockhash: BytesVec::forced_import(db, "blockhash", version)?,
            height_to_difficulty: PcoVec::forced_import(db, "difficulty", version)?,
            height_to_timestamp: PcoVec::forced_import(db, "timestamp", version)?,
            height_to_total_size: PcoVec::forced_import(db, "total_size", version)?,
            height_to_weight: PcoVec::forced_import(db, "weight", version)?,
        })
    }

    pub fn truncate(&mut self, height: Height, stamp: Stamp) -> Result<()> {
        self.height_to_blockhash
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_difficulty
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_timestamp
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_total_size
            .truncate_if_needed_with_stamp(height, stamp)?;
        self.height_to_weight
            .truncate_if_needed_with_stamp(height, stamp)?;
        Ok(())
    }

    pub fn par_iter_mut_any(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        [
            &mut self.height_to_blockhash as &mut dyn AnyStoredVec,
            &mut self.height_to_difficulty,
            &mut self.height_to_timestamp,
            &mut self.height_to_total_size,
            &mut self.height_to_weight,
        ]
        .into_par_iter()
    }
}
