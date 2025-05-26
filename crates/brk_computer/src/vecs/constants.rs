use std::{fs, path::Path};

use brk_core::StoredU8;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, AnyVec, Compressed, Computation, Version};

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, StorableVecGeneatorOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub _0: ComputedVecsFromHeight<StoredU8>,
    pub _1: ComputedVecsFromHeight<StoredU8>,
    pub _50: ComputedVecsFromHeight<StoredU8>,
    pub _100: ComputedVecsFromHeight<StoredU8>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        _computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            _0: ComputedVecsFromHeight::forced_import(
                path,
                "0",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1: ComputedVecsFromHeight::forced_import(
                path,
                "1",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _50: ComputedVecsFromHeight::forced_import(
                path,
                "50",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _100: ComputedVecsFromHeight::forced_import(
                path,
                "100",
                true,
                version + VERSION + Version::ZERO,
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
        self._0.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU8::new(0)),
                    exit,
                )
            },
        )?;

        self._1.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU8::new(1)),
                    exit,
                )
            },
        )?;

        self._50.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU8::new(50)),
                    exit,
                )
            },
        )?;

        self._100.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU8::new(100)),
                    exit,
                )
            },
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self._0.vecs(),
            self._1.vecs(),
            self._50.vecs(),
            self._100.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
