use std::sync::Arc;

use brk_core::{DifficultyEpoch, HalvingEpoch, StoredF64, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vecs::{AnyCollectableVec, Computation, File, Format, VecIterator};

use crate::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_difficulty: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_difficultyepoch: ComputedVecsFromDateIndex<DifficultyEpoch>,
    pub indexes_to_halvingepoch: ComputedVecsFromDateIndex<HalvingEpoch>,
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
            indexes_to_difficulty: ComputedVecsFromHeight::forced_import(
                file,
                "difficulty",
                Source::None,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_difficultyepoch: ComputedVecsFromDateIndex::forced_import(
                file,
                "difficultyepoch",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_halvingepoch: ComputedVecsFromDateIndex::forced_import(
                file,
                "halvingepoch",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
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
        let mut height_to_difficultyepoch_iter = indexes.height_to_difficultyepoch.into_iter();
        self.indexes_to_difficultyepoch.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_difficultyepoch_iter.unwrap_get_inner(
                                height + (*height_count_iter.unwrap_get_inner(di) - 1),
                            ),
                        )
                    },
                    exit,
                )
            },
        )?;

        let mut height_to_halvingepoch_iter = indexes.height_to_halvingepoch.into_iter();
        self.indexes_to_halvingepoch.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                let mut height_count_iter = indexes.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_halvingepoch_iter.unwrap_get_inner(
                                height + (*height_count_iter.unwrap_get_inner(di) - 1),
                            ),
                        )
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_difficulty.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.height_to_difficulty),
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes_to_difficulty.vecs(),
            self.indexes_to_difficultyepoch.vecs(),
            self.indexes_to_halvingepoch.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
