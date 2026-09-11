use bitview_collections::PerResolution;
use bitview_plugin_indexer::Indexer;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{
    BLOCKS_PER_DIFF_EPOCHS, BLOCKS_PER_HALVING, Day1, Day3, Epoch, Halving, Height, Hour1, Hour4,
    Hour12, Minute10, Minute30, Month1, Month3, Month6, Timestamp, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec, LazyVec, PcoVec,
    ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Rw, StorageMode, Version,
};

use super::{DatedResolutionVecs, ResolutionVecs};

mod boundary;

pub use boundary::BoundaryTimestampVec;

/// Timestamps: monotonic height→timestamp + per-period timestamp lookups.
///
/// Time-based periods (minute10–year10) are lazy: `idx.to_timestamp()` is a pure
/// function of the index, so no storage or decompression is needed.
/// Block-based periods (halving, difficulty) are storage-free views of the raw
/// timestamp at each period's first block.
#[derive(Deref, DerefMut, Traversable)]
pub struct Timestamps<M: StorageMode = Rw> {
    /// Nondecreasing Unix timestamp in seconds at each block height, computed as
    /// the maximum of the represented raw block-header timestamp and the preceding
    /// monotonic timestamp.
    pub monotonic: M::Stored<EagerVec<PcoVec<Height, Timestamp, Budgeted>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: PerResolution<
        LazyVec<Minute10, Timestamp, Minute10, Height>,
        LazyVec<Minute30, Timestamp, Minute30, Height>,
        LazyVec<Hour1, Timestamp, Hour1, Height>,
        LazyVec<Hour4, Timestamp, Hour4, Height>,
        LazyVec<Hour12, Timestamp, Hour12, Height>,
        LazyVec<Day1, Timestamp, Day1, Height>,
        LazyVec<Day3, Timestamp, Day3, Height>,
        LazyVec<Week1, Timestamp, Week1, Height>,
        LazyVec<Month1, Timestamp, Month1, Height>,
        LazyVec<Month3, Timestamp, Month3, Height>,
        LazyVec<Month6, Timestamp, Month6, Height>,
        LazyVec<Year1, Timestamp, Year1, Height>,
        LazyVec<Year10, Timestamp, Year10, Height>,
        BoundaryTimestampVec<Halving>,
        BoundaryTimestampVec<Epoch>,
    >,
}

impl Timestamps {
    pub fn forced_import_monotonic(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
    ) -> Result<EagerVec<PcoVec<Height, Timestamp, Budgeted>>> {
        Ok(EagerVec::forced_import_with(
            ImportOptions::new(db, "timestamp_monotonic", version).with_cache_budget(cache),
        )?)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_locals(
        version: Version,
        monotonic: EagerVec<PcoVec<Height, Timestamp, Budgeted>>,
        raw_timestamps: ReadableBoxedVec<Height, Timestamp>,
        minute10: &ResolutionVecs<Minute10>,
        minute30: &ResolutionVecs<Minute30>,
        hour1: &ResolutionVecs<Hour1>,
        hour4: &ResolutionVecs<Hour4>,
        hour12: &ResolutionVecs<Hour12>,
        day1: &DatedResolutionVecs<Day1>,
        day3: &DatedResolutionVecs<Day3>,
        week1: &DatedResolutionVecs<Week1>,
        month1: &DatedResolutionVecs<Month1>,
        month3: &DatedResolutionVecs<Month3>,
        month6: &DatedResolutionVecs<Month6>,
        year1: &DatedResolutionVecs<Year1>,
        year10: &DatedResolutionVecs<Year10>,
    ) -> Self {
        macro_rules! period {
            ($field:ident) => {
                LazyVec::init(
                    "timestamp",
                    version,
                    $field.first_height.read_only_boxed_clone(),
                    |idx, _: Height| idx.to_timestamp(),
                )
            };
        }

        Self {
            monotonic,
            resolutions: PerResolution {
                minute10: period!(minute10),
                minute30: period!(minute30),
                hour1: period!(hour1),
                hour4: period!(hour4),
                hour12: period!(hour12),
                day1: period!(day1),
                day3: period!(day3),
                week1: period!(week1),
                month1: period!(month1),
                month3: period!(month3),
                month6: period!(month6),
                year1: period!(year1),
                year10: period!(year10),
                halving: BoundaryTimestampVec::new(
                    raw_timestamps.clone(),
                    BLOCKS_PER_HALVING as usize,
                ),
                epoch: BoundaryTimestampVec::new(raw_timestamps, BLOCKS_PER_DIFF_EPOCHS as usize),
            },
        }
    }

    pub fn compute_monotonic(
        &mut self,
        indexer: &Indexer,
        starting_height: Height,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev = None;
        self.monotonic.compute_transform(
            starting_height,
            &indexer.vecs().blocks.timestamp,
            |(h, timestamp, this)| {
                if prev.is_none()
                    && let Some(prev_h) = h.decremented()
                {
                    prev.replace(this.collect_one(prev_h).unwrap());
                }
                let monotonic = prev.map_or(timestamp, |p| p.max(timestamp));
                prev.replace(monotonic);
                (h, monotonic)
            },
            exit,
        )?;
        Ok(())
    }
}
