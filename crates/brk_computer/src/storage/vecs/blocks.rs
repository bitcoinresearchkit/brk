use std::{fs, path::Path};

use brk_core::{CheckedSub, Dateindex, Height, Timestamp};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

use super::{
    Indexes, StorableVec, indexes,
    stats::{StorableVecGeneatorByIndex, StorableVecGeneatorOptions},
};

#[derive(Clone)]
pub struct Vecs {
    pub height_to_block_interval: StorableVec<Height, Timestamp>,
    pub indexes_to_block_interval_stats: StorableVecGeneatorByIndex<Timestamp>,
    pub dateindex_to_block_count: StorableVec<Dateindex, u16>,
    pub dateindex_to_total_block_count: StorableVec<Dateindex, u32>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            height_to_block_interval: StorableVec::forced_import(
                &path.join("height_to_block_interval"),
                Version::from(1),
                compressed,
            )?,
            indexes_to_block_interval_stats: StorableVecGeneatorByIndex::forced_import(
                &path.join("block_interval"),
                compressed,
                StorableVecGeneatorOptions::default()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            dateindex_to_block_count: StorableVec::forced_import(
                &path.join("dateindex_to_block_count"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_total_block_count: StorableVec::forced_import(
                &path.join("dateindex_to_total_block_count"),
                Version::from(1),
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

        self.height_to_block_interval.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(height, timestamp, _, height_to_timestamp)| {
                let interval = height.decremented().map_or(Timestamp::ZERO, |prev_h| {
                    let prev_timestamp = *height_to_timestamp.get(prev_h).unwrap().unwrap();
                    dbg!((timestamp, prev_timestamp));
                    timestamp
                        .checked_sub(prev_timestamp)
                        .unwrap_or(Timestamp::ZERO)
                });
                dbg!((height, interval));
                (height, interval)
            },
            exit,
        )?;

        self.indexes_to_block_interval_stats.compute(
            &mut self.height_to_block_interval,
            indexes,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            vec![
                self.height_to_block_interval.any_vec(),
                self.dateindex_to_block_count.any_vec(),
                self.dateindex_to_total_block_count.any_vec(),
            ],
            self.indexes_to_block_interval_stats.as_any_vecs(),
        ]
        .concat()
    }
}
