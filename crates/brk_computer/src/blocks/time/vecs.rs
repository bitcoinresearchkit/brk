use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Date, Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour4, Hour12, Indexes,
    Minute10, Minute30, Month1, Month3, Month6, Timestamp, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use vecdb::{EagerVec, Exit, LazyVecFrom1, PcoVec, ReadableVec, Rw, StorageMode};

use crate::{indexes, internal::PerResolution};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub date: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub timestamp_monotonic: M::Stored<EagerVec<PcoVec<Height, Timestamp>>>,
    pub timestamp: TimestampIndexes<M>,
}

/// Per-period timestamp indexes.
///
/// Time-based periods (minute10–year10) are lazy: `idx.to_timestamp()` is a pure
/// function of the index, so no storage or decompression is needed.
/// Epoch-based periods (halving, difficulty) are eager: their timestamps
/// come from block data via `compute_indirect_sequential`.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct TimestampIndexes<M: StorageMode = Rw>(
    #[allow(clippy::type_complexity)]
    pub  PerResolution<
        LazyVecFrom1<Minute10, Timestamp, Minute10, Height>,
        LazyVecFrom1<Minute30, Timestamp, Minute30, Height>,
        LazyVecFrom1<Hour1, Timestamp, Hour1, Height>,
        LazyVecFrom1<Hour4, Timestamp, Hour4, Height>,
        LazyVecFrom1<Hour12, Timestamp, Hour12, Height>,
        LazyVecFrom1<Day1, Timestamp, Day1, Height>,
        LazyVecFrom1<Day3, Timestamp, Day3, Height>,
        LazyVecFrom1<Week1, Timestamp, Week1, Height>,
        LazyVecFrom1<Month1, Timestamp, Month1, Height>,
        LazyVecFrom1<Month3, Timestamp, Month3, Height>,
        LazyVecFrom1<Month6, Timestamp, Month6, Height>,
        LazyVecFrom1<Year1, Timestamp, Year1, Height>,
        LazyVecFrom1<Year10, Timestamp, Year10, Height>,
        M::Stored<EagerVec<PcoVec<HalvingEpoch, Timestamp>>>,
        M::Stored<EagerVec<PcoVec<DifficultyEpoch, Timestamp>>>,
    >,
);

impl TimestampIndexes {
    /// Compute epoch timestamps via indirect lookup from block timestamps.
    /// Time-based periods are lazy (idx.to_timestamp()) and need no compute.
    pub(crate) fn compute(
        &mut self,
        indexer: &brk_indexer::Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let prev_height = starting_indexes.height.decremented().unwrap_or_default();
        self.halving.compute_indirect_sequential(
            indexes
                .height
                .halving
                .collect_one(prev_height)
                .unwrap_or_default(),
            &indexes.halving.first_height,
            &indexer.vecs.blocks.timestamp,
            exit,
        )?;
        self.difficulty.compute_indirect_sequential(
            indexes
                .height
                .difficulty
                .collect_one(prev_height)
                .unwrap_or_default(),
            &indexes.difficulty.first_height,
            &indexer.vecs.blocks.timestamp,
            exit,
        )?;
        Ok(())
    }
}
