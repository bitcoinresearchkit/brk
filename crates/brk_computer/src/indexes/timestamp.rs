use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, Epoch, Halving, Height, Hour1, Hour4, Hour12, Indexes, Minute10, Minute30, Month1,
    Month3, Month6, Timestamp, Week1, Year1, Year10,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, LazyVecFrom1, PcoVec, ReadableVec, Rw, StorageMode,
    Version,
};

use crate::internal::PerResolution;

/// Timestamps: monotonic height→timestamp + per-period timestamp lookups.
///
/// Time-based periods (minute10–year10) are lazy: `idx.to_timestamp()` is a pure
/// function of the index, so no storage or decompression is needed.
/// Epoch-based periods (halving, difficulty) are eager: their timestamps
/// come from block data via `compute_indirect_sequential`.
#[derive(Deref, DerefMut, Traversable)]
pub struct Timestamps<M: StorageMode = Rw> {
    pub monotonic: M::Stored<EagerVec<PcoVec<Height, Timestamp>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    #[allow(clippy::type_complexity)]
    pub resolutions: PerResolution<
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
        M::Stored<EagerVec<PcoVec<Halving, Timestamp>>>,
        M::Stored<EagerVec<PcoVec<Epoch, Timestamp>>>,
    >,
}

impl Timestamps {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import_from_locals(
        db: &Database,
        version: Version,
        minute10: &super::Minute10Vecs,
        minute30: &super::Minute30Vecs,
        hour1: &super::Hour1Vecs,
        hour4: &super::Hour4Vecs,
        hour12: &super::Hour12Vecs,
        day1: &super::Day1Vecs,
        day3: &super::Day3Vecs,
        week1: &super::Week1Vecs,
        month1: &super::Month1Vecs,
        month3: &super::Month3Vecs,
        month6: &super::Month6Vecs,
        year1: &super::Year1Vecs,
        year10: &super::Year10Vecs,
    ) -> Result<Self> {
        let monotonic = EagerVec::forced_import(db, "timestamp_monotonic", version)?;

        macro_rules! period {
            ($field:ident) => {
                LazyVecFrom1::init(
                    "timestamp",
                    version,
                    $field.first_height.read_only_boxed_clone(),
                    |idx, _: Height| idx.to_timestamp(),
                )
            };
        }

        Ok(Self {
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
                halving: ImportableVec::forced_import(db, "timestamp", version)?,
                epoch: ImportableVec::forced_import(db, "timestamp", version)?,
            },
        })
    }

    pub(crate) fn compute_monotonic(
        &mut self,
        indexer: &brk_indexer::Indexer,
        starting_height: Height,
        exit: &Exit,
    ) -> Result<()> {
        let mut prev = None;
        self.monotonic.compute_transform(
            starting_height,
            &indexer.vecs.blocks.timestamp,
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

    pub(crate) fn compute_per_resolution(
        &mut self,
        indexer: &brk_indexer::Indexer,
        height: &super::HeightVecs,
        halving_vecs: &super::HalvingVecs,
        epoch_vecs: &super::EpochVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let prev_height = starting_indexes.height.decremented().unwrap_or_default();
        self.resolutions.halving.compute_indirect_sequential(
            height.halving.collect_one(prev_height).unwrap_or_default(),
            &halving_vecs.first_height,
            &indexer.vecs.blocks.timestamp,
            exit,
        )?;
        self.resolutions.epoch.compute_indirect_sequential(
            height.epoch.collect_one(prev_height).unwrap_or_default(),
            &epoch_vecs.first_height,
            &indexer.vecs.blocks.timestamp,
            exit,
        )?;
        Ok(())
    }
}
