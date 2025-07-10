use std::path::Path;

use brk_core::{StoredU8, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, AnyVec, Computation, Format};

use crate::vecs::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub constant_0: ComputedVecsFromHeight<StoredU8>,
    pub constant_1: ComputedVecsFromHeight<StoredU8>,
    pub constant_50: ComputedVecsFromHeight<StoredU8>,
    pub constant_100: ComputedVecsFromHeight<StoredU8>,
}

impl Vecs {
    pub fn forced_import(
        path: &Path,
        version: Version,
        computation: Computation,
        format: Format,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            constant_0: ComputedVecsFromHeight::forced_import(
                path,
                "constant_0",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_1: ComputedVecsFromHeight::forced_import(
                path,
                "constant_1",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_50: ComputedVecsFromHeight::forced_import(
                path,
                "constant_50",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_100: ComputedVecsFromHeight::forced_import(
                path,
                "constant_100",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
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
        self.constant_0.compute_all(
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

        self.constant_1.compute_all(
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

        self.constant_50.compute_all(
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

        self.constant_100.compute_all(
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
            self.constant_0.vecs(),
            self.constant_1.vecs(),
            self.constant_50.vecs(),
            self.constant_100.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
