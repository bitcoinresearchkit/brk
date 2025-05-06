use std::{fs, path::Path};

use brk_core::{
    CheckedSub, DifficultyEpoch, HalvingEpoch, Height, StoredU32, StoredU64, StoredUsize,
    Timestamp, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{AnyCollectableVec, AnyIterableVec, Compressed, Computation, EagerVec, Version};

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateindex, ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    pub height_to_interval: EagerVec<Height, Timestamp>,
    pub height_to_vbytes: EagerVec<Height, StoredU64>,
    pub difficultyepoch_to_timestamp: EagerVec<DifficultyEpoch, Timestamp>,
    pub halvingepoch_to_timestamp: EagerVec<HalvingEpoch, Timestamp>,
    pub timeindexes_to_timestamp: ComputedVecsFromDateindex<Timestamp>,
    pub indexes_to_block_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredUsize>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_weight: ComputedVecsFromHeight<Weight>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        _computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            height_to_interval: EagerVec::forced_import(
                &path.join("height_to_interval"),
                Version::ZERO,
                compressed,
            )?,
            timeindexes_to_timestamp: ComputedVecsFromDateindex::forced_import(
                path,
                "timestamp",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_first(),
            )?,
            indexes_to_block_interval: ComputedVecsFromHeight::forced_import(
                path,
                "block_interval",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_block_count: ComputedVecsFromHeight::forced_import(
                path,
                "block_count",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_block_weight: ComputedVecsFromHeight::forced_import(
                path,
                "block_weight",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            indexes_to_block_size: ComputedVecsFromHeight::forced_import(
                path,
                "block_size",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            height_to_vbytes: EagerVec::forced_import(
                &path.join("height_to_vbytes"),
                Version::ZERO,
                compressed,
            )?,
            indexes_to_block_vbytes: ComputedVecsFromHeight::forced_import(
                path,
                "block_vbytes",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
            difficultyepoch_to_timestamp: EagerVec::forced_import(
                &path.join("difficultyepoch_to_timestamp"),
                Version::ZERO,
                compressed,
            )?,
            halvingepoch_to_timestamp: EagerVec::forced_import(
                &path.join("halvingepoch_to_timestamp"),
                Version::ZERO,
                compressed,
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
        self.timeindexes_to_timestamp.compute(
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
                let indexer_vecs = indexer.vecs();

                v.compute_range(
                    starting_indexes.height,
                    &indexer_vecs.height_to_weight,
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )
            },
        )?;

        let indexer_vecs = indexer.vecs();

        let mut height_to_timestamp_iter = indexer_vecs.height_to_timestamp.iter();
        self.height_to_interval.compute_transform(
            starting_indexes.height,
            &indexer_vecs.height_to_timestamp,
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
            Some(&indexer_vecs.height_to_weight),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer_vecs.height_to_total_size),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            &indexer_vecs.height_to_weight,
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

        let mut height_to_timestamp_iter = indexer_vecs.height_to_timestamp.iter();

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
        .concat()
    }
}
