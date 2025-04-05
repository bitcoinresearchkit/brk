use std::{fs, path::Path};

use brk_core::{CheckedSub, Dateindex, Timestamp};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

use super::{
    ComputedVec, Indexes,
    grouped::{ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_block_interval: ComputedVecsFromHeight<Timestamp>,
    pub dateindex_to_block_count: ComputedVec<Dateindex, u16>,
    pub dateindex_to_total_block_count: ComputedVec<Dateindex, u32>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            indexes_to_block_interval: ComputedVecsFromHeight::forced_import(
                path,
                "block_interval",
                Version::ONE,
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            dateindex_to_block_count: ComputedVec::forced_import(
                &path.join("dateindex_to_block_count"),
                Version::ONE,
                compressed,
            )?,
            dateindex_to_total_block_count: ComputedVec::forced_import(
                &path.join("dateindex_to_total_block_count"),
                Version::ONE,
                compressed,
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
        let indexer_vecs = indexer.mut_vecs();

        self.indexes_to_block_interval.compute(
            |v| {
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
            indexes,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            vec![
                self.dateindex_to_block_count.any_vec(),
                self.dateindex_to_total_block_count.any_vec(),
            ],
            self.indexes_to_block_interval.any_vecs(),
        ]
        .concat()
    }
}
