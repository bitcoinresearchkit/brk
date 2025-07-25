use std::sync::Arc;

use brk_core::{
    CheckedSub, DifficultyEpoch, HalvingEpoch, Height, StoredU32, StoredU64, StoredUsize,
    Timestamp, Version, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vecs::{AnyCollectableVec, AnyIterableVec, Computation, EagerVec, File, Format};

use crate::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub height_to_interval: EagerVec<Height, Timestamp>,
    pub height_to_vbytes: EagerVec<Height, StoredU64>,
    pub difficultyepoch_to_timestamp: EagerVec<DifficultyEpoch, Timestamp>,
    pub halvingepoch_to_timestamp: EagerVec<HalvingEpoch, Timestamp>,
    pub timeindexes_to_timestamp: ComputedVecsFromDateIndex<Timestamp>,
    pub indexes_to_block_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_weight: ComputedVecsFromHeight<Weight>,
}

impl Vecs {
    pub fn forced_import(
        file: &Arc<File>,
        version: Version,
        computation: Computation,
        format: Format,
        indexes: &indexes::Vecs,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            height_to_interval: EagerVec::forced_import(
                file,
                "interval",
                version + VERSION + Version::ZERO,
                format,
            )?,
            timeindexes_to_timestamp: ComputedVecsFromDateIndex::forced_import(
                file,
                "timestamp",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_first(),
            )?,
            indexes_to_block_interval: ComputedVecsFromHeight::forced_import(
                file,
                "block_interval",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_block_count: ComputedVecsFromHeight::forced_import(
                file,
                "block_count",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_block_weight: ComputedVecsFromHeight::forced_import(
                file,
                "block_weight",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_block_size: ComputedVecsFromHeight::forced_import(
                file,
                "block_size",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            height_to_vbytes: EagerVec::forced_import(
                file,
                "vbytes",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_block_vbytes: ComputedVecsFromHeight::forced_import(
                file,
                "block_vbytes",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            difficultyepoch_to_timestamp: EagerVec::forced_import(
                file,
                "timestamp",
                version + VERSION + Version::ZERO,
                format,
            )?,
            halvingepoch_to_timestamp: EagerVec::forced_import(
                file,
                "timestamp",
                version + VERSION + Version::ZERO,
                format,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.timeindexes_to_timestamp.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_date,
                    |(di, d, ..)| (di, Timestamp::from(d)),
                    exit,
                )
            },
        )?;

        self.indexes_to_block_count.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_range(
                    starting_indexes.height,
                    &indexer.vecs.height_to_weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )
            },
        )?;

        let mut height_to_timestamp_iter = indexer.vecs.height_to_timestamp.iter();
        self.height_to_interval.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_timestamp,
            |(height, timestamp, ..)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = height_to_timestamp_iter.unwrap_get_inner(prev_h);
                    timestamp
                        .checked_sub(prev_timestamp)
                        .unwrap_or(Timestamp::ZERO)
                });
                (height, interval)
            },
            exit,
        )?;

        self.indexes_to_block_interval.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_interval),
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_weight),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_total_size),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            &indexer.vecs.height_to_weight,
            |(h, w, ..)| {
                (
                    h,
                    StoredU64::from(bitcoin::Weight::from(w).to_vbytes_floor()),
                )
            },
            exit,
        )?;

        self.indexes_to_block_vbytes.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&self.height_to_vbytes),
        )?;

        let mut height_to_timestamp_iter = indexer.vecs.height_to_timestamp.iter();

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            &indexes.difficultyepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.unwrap_get_inner(h)),
            exit,
        )?;

        self.halvingepoch_to_timestamp.compute_transform(
            starting_indexes.halvingepoch,
            &indexes.halvingepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.unwrap_get_inner(h)),
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![
                &self.height_to_interval as &dyn AnyCollectableVec,
                &self.height_to_vbytes,
                &self.difficultyepoch_to_timestamp,
                &self.halvingepoch_to_timestamp,
            ],
            self.timeindexes_to_timestamp.vecs(),
            self.indexes_to_block_count.vecs(),
            self.indexes_to_block_interval.vecs(),
            self.indexes_to_block_size.vecs(),
            self.indexes_to_block_vbytes.vecs(),
            self.indexes_to_block_weight.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
