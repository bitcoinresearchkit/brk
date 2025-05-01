use std::{fs, path::Path};

use brk_core::{DifficultyEpoch, HalvingEpoch, StoredF64};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{Compressed, VecIterator, Version};

use super::{
    Indexes,
    grouped::{ComputedVecsFromDateindex, ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes,
};

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_difficulty: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_difficultyepoch: ComputedVecsFromDateindex<DifficultyEpoch>,
    pub indexes_to_halvingepoch: ComputedVecsFromDateindex<HalvingEpoch>,
}

impl Vecs {
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            indexes_to_difficulty: ComputedVecsFromHeight::forced_import(
                path,
                "difficulty",
                false,
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_difficultyepoch: ComputedVecsFromDateindex::forced_import(
                path,
                "difficultyepoch",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_halvingepoch: ComputedVecsFromDateindex::forced_import(
                path,
                "halvingepoch",
                Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
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
        let mut height_to_difficultyepoch_iter = indexes.height_to_difficultyepoch.iter();
        self.indexes_to_difficultyepoch.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                let mut height_count_iter = indexes.dateindex_to_height_count.iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    indexes.dateindex_to_first_height.vec(),
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

        let mut height_to_halvingepoch_iter = indexes.height_to_halvingepoch.iter();
        self.indexes_to_halvingepoch.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                let mut height_count_iter = indexes.dateindex_to_height_count.iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    indexes.dateindex_to_first_height.vec(),
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
            Some(indexer.vecs().height_to_difficulty.vec()),
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        [
            self.indexes_to_difficulty.any_vecs(),
            self.indexes_to_difficultyepoch.any_vecs(),
            self.indexes_to_halvingepoch.any_vecs(),
        ]
        .concat()
    }
}
