use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{StoredI16, StoredU16, Version};
use vecdb::{AnyCollectableVec, AnyVec, Database, Exit};

use crate::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    db: Database,

    pub constant_0: ComputedVecsFromHeight<StoredU16>,
    pub constant_1: ComputedVecsFromHeight<StoredU16>,
    pub constant_2: ComputedVecsFromHeight<StoredU16>,
    pub constant_3: ComputedVecsFromHeight<StoredU16>,
    pub constant_4: ComputedVecsFromHeight<StoredU16>,
    pub constant_50: ComputedVecsFromHeight<StoredU16>,
    pub constant_100: ComputedVecsFromHeight<StoredU16>,
    pub constant_minus_1: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_2: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_3: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_4: ComputedVecsFromHeight<StoredI16>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let db = Database::open(&parent.join("constants"))?;

        Ok(Self {
            constant_0: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_0",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_1: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_1",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_2: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_2",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_3: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_3",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_4: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_4",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_50: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_50",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_100: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_100",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_1: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_1",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_2: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_2",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_3: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_3",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_4: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_4",
                Source::Compute,
                version + VERSION + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            db,
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
        self.db.flush_then_punch()?;
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

        self.constant_2.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU16::new(2)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.constant_3.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU16::new(3)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.constant_4.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU16::new(4)),
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

        self.constant_minus_1.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredI16::new(-1)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.constant_minus_2.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredI16::new(-2)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.constant_minus_3.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredI16::new(-3)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.constant_minus_4.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |vec, _, indexes, starting_indexes, exit| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredI16::new(-4)),
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
            self.constant_2.vecs(),
            self.constant_3.vecs(),
            self.constant_4.vecs(),
            self.constant_50.vecs(),
            self.constant_100.vecs(),
            self.constant_minus_1.vecs(),
            self.constant_minus_2.vecs(),
            self.constant_minus_3.vecs(),
            self.constant_minus_4.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
