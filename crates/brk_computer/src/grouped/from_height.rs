use allocative::Allocative;
use brk_error::Result;

use brk_structs::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use vecdb::{AnyCloneableIterableVec, AnyCollectableVec, AnyIterableVec, Database, EagerVec, Exit};

use crate::{
    Indexes,
    grouped::{LazyVecBuilder, Source},
    indexes,
};

use super::{ComputedType, EagerVecBuilder, VecBuilderOptions};

#[derive(Clone, Allocative)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: Option<EagerVec<Height, T>>,
    pub height_extra: EagerVecBuilder<Height, T>,
    pub dateindex: EagerVecBuilder<DateIndex, T>,
    pub weekindex: LazyVecBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: EagerVecBuilder<DifficultyEpoch, T>,
    pub monthindex: LazyVecBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecBuilder<YearIndex, T, DateIndex, YearIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: LazyVecBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeight<T>
where
    T: ComputedType + Ord + From<f64> + 'static,
    f64: From<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let height = source.is_compute().then(|| {
            EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO).unwrap()
        });

        let height_extra = EagerVecBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let dateindex = EagerVecBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options,
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            weekindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
            height,
            height_extra,
            dateindex,
            difficultyepoch: EagerVecBuilder::forced_import_compressed(
                db,
                name,
                version + VERSION + Version::ZERO,
                options,
            )?,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<Height, T>) -> Result<()>,
    {
        compute(self.height.as_mut().unwrap())?;

        let height: Option<&EagerVec<Height, T>> = None;
        self.compute_rest(indexes, starting_indexes, exit, height)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        height_vec: Option<&impl AnyIterableVec<Height, T>>,
    ) -> Result<()> {
        if let Some(height) = height_vec {
            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.dateindex_to_first_height,
                &indexes.dateindex_to_height_count,
                exit,
            )?;

            self.difficultyepoch.compute(
                starting_indexes.difficultyepoch,
                height,
                &indexes.difficultyepoch_to_first_height,
                &indexes.difficultyepoch_to_height_count,
                exit,
            )?;
        } else {
            let height = self.height.as_ref().unwrap();

            self.height_extra
                .extend(starting_indexes.height, height, exit)?;

            self.dateindex.compute(
                starting_indexes.dateindex,
                height,
                &indexes.dateindex_to_first_height,
                &indexes.dateindex_to_height_count,
                exit,
            )?;

            self.difficultyepoch.compute(
                starting_indexes.difficultyepoch,
                height,
                &indexes.difficultyepoch_to_first_height,
                &indexes.difficultyepoch_to_height_count,
                exit,
            )?;
        }

        Ok(())
    }

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> = Box::new(
            self.height
                .as_ref()
                .map(|x| x as &dyn AnyCollectableVec)
                .into_iter(),
        );

        iter = Box::new(iter.chain(self.height_extra.iter_any_collectable()));
        iter = Box::new(iter.chain(self.dateindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.weekindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.difficultyepoch.iter_any_collectable()));
        iter = Box::new(iter.chain(self.monthindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.quarterindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.semesterindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.yearindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.decadeindex.iter_any_collectable()));

        iter
    }
}
