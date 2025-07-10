use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, Result, SemesterIndex, Version, WeekIndex,
    YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, CloneableAnyIterableVec, Computation, EagerVec, Format,
};

use crate::vecs::{Indexes, grouped::ComputedVecBuilder, indexes};

use super::{ComputedType, EagerVecBuilder, Source, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromDateIndex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: Option<EagerVec<DateIndex, T>>,
    pub dateindex_extra: EagerVecBuilder<DateIndex, T>,
    pub weekindex: ComputedVecBuilder<WeekIndex, T, DateIndex>,
    pub monthindex: ComputedVecBuilder<MonthIndex, T, DateIndex>,
    pub quarterindex: ComputedVecBuilder<QuarterIndex, T, DateIndex>,
    pub semesterindex: ComputedVecBuilder<SemesterIndex, T, DateIndex>,
    pub yearindex: ComputedVecBuilder<YearIndex, T, DateIndex>,
    pub decadeindex: ComputedVecBuilder<DecadeIndex, T, DateIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromDateIndex<T>
where
    T: ComputedType + 'static,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        source: Source<DateIndex, T>,
        version: Version,
        format: Format,
        computation: Computation,
        options: VecBuilderOptions,
    ) -> color_eyre::Result<Self> {
        let dateindex = source.is_compute().then(|| {
            EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format).unwrap()
        });

        let dateindex_extra = EagerVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        let dateindex_source = source.vec().or(dateindex.as_ref().map(|v| v.boxed_clone()));

        Ok(Self {
            weekindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            semesterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            yearindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            decadeindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                dateindex_source.clone(),
                &dateindex_extra,
                options.into(),
            )?,
            dateindex,
            dateindex_extra,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut EagerVec<DateIndex, T>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.dateindex.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let dateindex: Option<&EagerVec<DateIndex, T>> = None;
        self.compute_rest(indexes, starting_indexes, exit, dateindex)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl AnyIterableVec<DateIndex, T>>,
    ) -> color_eyre::Result<()> {
        if let Some(dateindex) = dateindex {
            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        } else {
            let dateindex = self.dateindex.as_ref().unwrap();

            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;
        }

        self.weekindex.compute_if_necessary(
            starting_indexes.weekindex,
            &indexes.weekindex_to_dateindex_count,
            exit,
        )?;

        self.monthindex.compute_if_necessary(
            starting_indexes.monthindex,
            &indexes.monthindex_to_dateindex_count,
            exit,
        )?;

        self.quarterindex.compute_if_necessary(
            starting_indexes.quarterindex,
            &indexes.quarterindex_to_monthindex_count,
            exit,
        )?;

        self.semesterindex.compute_if_necessary(
            starting_indexes.semesterindex,
            &indexes.semesterindex_to_monthindex_count,
            exit,
        )?;

        self.yearindex.compute_if_necessary(
            starting_indexes.yearindex,
            &indexes.yearindex_to_monthindex_count,
            exit,
        )?;

        self.decadeindex.compute_if_necessary(
            starting_indexes.decadeindex,
            &indexes.decadeindex_to_yearindex_count,
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.dateindex
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_extra.vecs(),
            self.weekindex.vecs(),
            self.monthindex.vecs(),
            self.quarterindex.vecs(),
            self.semesterindex.vecs(),
            self.yearindex.vecs(),
            self.decadeindex.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
