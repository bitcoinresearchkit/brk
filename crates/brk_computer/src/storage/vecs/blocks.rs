use std::{fs, path::Path};

use brk_core::{
    CheckedSub, DifficultyEpoch, HalvingEpoch, Height, StoredU32, StoredU64, StoredUsize,
    Timestamp, Weight,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{Compressed, Version};
use color_eyre::eyre::ContextCompat;

use super::{
    EagerVec, Indexes,
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
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
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
                    indexes.dateindex_to_date.vec(),
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
                    indexer_vecs.height_to_weight.vec(),
                    |h| (h, StoredU32::from(1_u32)),
                    exit,
                )
            },
        )?;

        let indexer_vecs = indexer.vecs();

        self.height_to_interval.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.vec(),
            |(height, timestamp, _, height_to_timestamp_iter)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = height_to_timestamp_iter
                        .get(prev_h)
                        .context("To work")
                        .inspect_err(|_| {
                            dbg!(prev_h);
                        })
                        .unwrap()
                        .1
                        .into_inner();
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
            Some(self.height_to_interval.vec()),
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(indexer_vecs.height_to_weight.vec()),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(indexer_vecs.height_to_total_size.vec()),
        )?;

        self.height_to_vbytes.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_weight.vec(),
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
            Some(self.height_to_vbytes.vec()),
        )?;

        let mut height_to_timestamp_iter = indexer_vecs.height_to_timestamp.iter();

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            indexes.difficultyepoch_to_first_height.vec(),
            |(i, h, ..)| (i, height_to_timestamp_iter.get(h).unwrap().1.into_inner()),
            exit,
        )?;

        self.halvingepoch_to_timestamp.compute_transform(
            starting_indexes.halvingepoch,
            indexes.halvingepoch_to_first_height.vec(),
            |(i, h, ..)| (i, height_to_timestamp_iter.get(h).unwrap().1.into_inner()),
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        [
            vec![
                self.height_to_interval.any_vec(),
                self.height_to_vbytes.any_vec(),
                self.difficultyepoch_to_timestamp.any_vec(),
                self.halvingepoch_to_timestamp.any_vec(),
            ],
            self.timeindexes_to_timestamp.any_vecs(),
            self.indexes_to_block_count.any_vecs(),
            self.indexes_to_block_interval.any_vecs(),
            self.indexes_to_block_size.any_vecs(),
            self.indexes_to_block_vbytes.any_vecs(),
            self.indexes_to_block_weight.any_vecs(),
        ]
        .concat()
    }
}
