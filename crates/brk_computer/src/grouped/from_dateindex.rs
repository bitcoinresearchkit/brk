use allocative::Allocative;
use brk_error::Result;

use brk_structs::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{AnyCloneableIterableVec, AnyCollectableVec, AnyIterableVec, Database, EagerVec, Exit};

use crate::{Indexes, grouped::LazyVecBuilder, indexes};

use super::{ComputedType, EagerVecBuilder, Source, VecBuilderOptions};

#[derive(Clone, Allocative)]
pub struct ComputedVecsFromDateIndex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: Option<EagerVec<DateIndex, T>>,
    pub dateindex_extra: EagerVecBuilder<DateIndex, T>,
    pub weekindex: LazyVecBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub monthindex: LazyVecBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecBuilder<YearIndex, T, DateIndex, YearIndex>,
    pub decadeindex: LazyVecBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromDateIndex<T>
where
    T: ComputedType + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<DateIndex, T>,
        version: Version,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let dateindex = source.is_compute().then(|| {
            EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO).unwrap()
        });

        let dateindex_extra = EagerVecBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        let dateindex_source = source.vec().or(dateindex.as_ref().map(|v| v.boxed_clone()));

        Ok(Self {
            weekindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                dateindex_source.clone(),
                &dateindex_extra,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),
            dateindex,
            dateindex_extra,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<DateIndex, T>) -> Result<()>,
    {
        compute(self.dateindex.as_mut().unwrap())?;

        let dateindex: Option<&EagerVec<DateIndex, T>> = None;
        self.compute_rest(starting_indexes, exit, dateindex)
    }

    pub fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl AnyIterableVec<DateIndex, T>>,
    ) -> Result<()> {
        if let Some(dateindex) = dateindex {
            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        } else {
            let dateindex = self.dateindex.as_ref().unwrap();

            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        }

        Ok(())
    }

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> = Box::new(
            self.dateindex
                .as_ref()
                .map(|x| x as &dyn AnyCollectableVec)
                .into_iter(),
        );

        iter = Box::new(iter.chain(self.dateindex_extra.iter_any_collectable()));
        iter = Box::new(iter.chain(self.weekindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.monthindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.quarterindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.semesterindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.yearindex.iter_any_collectable()));
        iter = Box::new(iter.chain(self.decadeindex.iter_any_collectable()));

        iter
    }
}
