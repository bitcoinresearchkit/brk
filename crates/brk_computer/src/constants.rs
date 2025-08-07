use std::{path::Path, sync::Arc};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{StoredU16, Version};
use brk_vecs::{AnyCollectableVec, AnyVec, Computation, Exit, File, Format};

use crate::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    file: Arc<File>,

    pub constant_0: ComputedVecsFromHeight<StoredU16>,
    pub constant_1: ComputedVecsFromHeight<StoredU16>,
    pub constant_50: ComputedVecsFromHeight<StoredU16>,
    pub constant_100: ComputedVecsFromHeight<StoredU16>,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        computation: Computation,
        format: Format,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let file = Arc::new(File::open(&parent.join("constants"))?);

        Ok(Self {
            constant_0: ComputedVecsFromHeight::forced_import(
                &file,
                "constant_0",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_1: ComputedVecsFromHeight::forced_import(
                &file,
                "constant_1",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_50: ComputedVecsFromHeight::forced_import(
                &file,
                "constant_50",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_100: ComputedVecsFromHeight::forced_import(
                &file,
                "constant_100",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            file,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexer, indexes, starting_indexes, exit)?;
        self.file.flush_then_punch()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
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
                    |i| (i, StoredU16::new(0)),
                    exit,
                )?;
                Ok(())
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
                    |i| (i, StoredU16::new(1)),
                    exit,
                )?;
                Ok(())
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
                    |i| (i, StoredU16::new(50)),
                    exit,
                )?;
                Ok(())
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
                    |i| (i, StoredU16::new(100)),
                    exit,
                )?;
                Ok(())
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
