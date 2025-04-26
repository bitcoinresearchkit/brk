use std::{fs, path::Path};

use brk_core::{DifficultyEpoch, HalvingEpoch, StoredF64};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{Compressed, DynamicVec, Version};

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
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_difficultyepoch.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    indexes.dateindex_to_last_height.mut_vec(),
                    |(di, height, ..)| {
                        (
                            di,
                            indexes
                                .height_to_difficultyepoch
                                .mut_vec()
                                .double_unwrap_cached_get(height),
                        )
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_halvingepoch.compute(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    indexes.dateindex_to_last_height.mut_vec(),
                    |(di, height, ..)| {
                        (
                            di,
                            indexes
                                .height_to_halvingepoch
                                .mut_vec()
                                .double_unwrap_cached_get(height),
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
            Some(indexer.mut_vecs().height_to_difficulty.mut_vec()),
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
