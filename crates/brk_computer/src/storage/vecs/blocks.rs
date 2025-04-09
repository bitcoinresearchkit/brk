use std::{fs, path::Path};

use brk_core::{CheckedSub, StoredU32, StoredU64, StoredUsize, Timestamp, Weight};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::bitcoin;
use brk_vec::{AnyStoredVec, Compressed, Version};

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub indexes_to_block_count: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_block_weight: ComputedVecsFromHeight<Weight>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredUsize>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            indexes_to_block_interval: ComputedVecsFromHeight::forced_import(
                path,
                "block_interval",
                true,
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
            indexes_to_block_vbytes: ComputedVecsFromHeight::forced_import(
                path,
                "block_vbytes",
                true,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_sum().add_total(),
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_block_interval.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                let indexer_vecs = indexer.mut_vecs();

                v.compute_transform(
                    starting_indexes.height,
                    indexer_vecs.height_to_timestamp.mut_vec(),
                    |(height, timestamp, _, height_to_timestamp)| {
                        let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                            let prev_timestamp = *height_to_timestamp.get(prev_h).unwrap().unwrap();
                            timestamp
                                .checked_sub(prev_timestamp)
                                .unwrap_or(Timestamp::ZERO)
                        });
                        (height, interval)
                    },
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
                v.compute_transform(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_block_weight.mut_vec(),
                    |(h, ..)| (h, StoredU32::from(1_u32)),
                    exit,
                )
            },
        )?;

        self.indexes_to_block_weight.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(indexer.mut_vecs().height_to_block_weight.mut_vec()),
        )?;

        self.indexes_to_block_size.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(indexer.mut_vecs().height_to_block_size.mut_vec()),
        )?;

        self.indexes_to_block_vbytes.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, indexer, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.height,
                    indexer.mut_vecs().height_to_block_weight.mut_vec(),
                    |(h, w, ..)| {
                        (
                            h,
                            StoredU64::from(bitcoin::Weight::from(w).to_vbytes_floor()),
                        )
                    },
                    exit,
                )
            },
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            self.indexes_to_block_interval.any_vecs(),
            self.indexes_to_block_count.any_vecs(),
            self.indexes_to_block_weight.any_vecs(),
            self.indexes_to_block_size.any_vecs(),
            self.indexes_to_block_vbytes.any_vecs(),
        ]
        .concat()
    }
}
