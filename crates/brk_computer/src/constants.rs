use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredI16, StoredU16, Version};
use vecdb::{AnyVec, Database, Exit, PAGE_SIZE};

use crate::grouped::Source;

use super::{
    Indexes,
    grouped::{ComputedVecsFromHeight, VecBuilderOptions},
    indexes,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub constant_0: ComputedVecsFromHeight<StoredU16>,
    pub constant_1: ComputedVecsFromHeight<StoredU16>,
    pub constant_2: ComputedVecsFromHeight<StoredU16>,
    pub constant_3: ComputedVecsFromHeight<StoredU16>,
    pub constant_4: ComputedVecsFromHeight<StoredU16>,
    pub constant_38_2: ComputedVecsFromHeight<StoredF32>,
    pub constant_50: ComputedVecsFromHeight<StoredU16>,
    pub constant_61_8: ComputedVecsFromHeight<StoredF32>,
    pub constant_100: ComputedVecsFromHeight<StoredU16>,
    pub constant_600: ComputedVecsFromHeight<StoredU16>,
    pub constant_minus_1: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_2: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_3: ComputedVecsFromHeight<StoredI16>,
    pub constant_minus_4: ComputedVecsFromHeight<StoredI16>,
}

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("constants"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + Version::ZERO;

        let this = Self {
            constant_0: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_0",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_1: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_1",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_2: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_2",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_3: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_3",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_4: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_4",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_38_2: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_38_2",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_50: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_50",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_61_8: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_61_8",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_100: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_100",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_600: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_600",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_1: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_1",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_2: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_2",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_3: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_3",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            constant_minus_4: ComputedVecsFromHeight::forced_import(
                &db,
                "constant_minus_4",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            db,
        };

        this.db.retain_regions(
            this.iter_any_writable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(indexes, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        [
            (&mut self.constant_0, 0),
            (&mut self.constant_1, 1),
            (&mut self.constant_2, 2),
            (&mut self.constant_3, 3),
            (&mut self.constant_4, 4),
            (&mut self.constant_50, 50),
            (&mut self.constant_100, 100),
            (&mut self.constant_600, 600),
        ]
        .into_iter()
        .try_for_each(|(vec, value)| {
            vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredU16::new(value)),
                    exit,
                )?;
                Ok(())
            })
        })?;

        [
            (&mut self.constant_minus_1, -1),
            (&mut self.constant_minus_2, -2),
            (&mut self.constant_minus_3, 3),
            (&mut self.constant_minus_4, 4),
        ]
        .into_iter()
        .try_for_each(|(vec, value)| {
            vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredI16::new(value)),
                    exit,
                )?;
                Ok(())
            })
        })?;

        [
            (&mut self.constant_38_2, 38.2),
            (&mut self.constant_61_8, 61.8),
        ]
        .into_iter()
        .try_for_each(|(vec, value)| {
            vec.compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_to(
                    starting_indexes.height,
                    indexes.height_to_date.len(),
                    indexes.height_to_date.version(),
                    |i| (i, StoredF32::from(value)),
                    exit,
                )?;
                Ok(())
            })
        })?;

        Ok(())
    }
}
