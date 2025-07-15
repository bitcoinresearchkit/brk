use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, Result,
    SemesterIndex, Version, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyIterableVec, CloneableAnyIterableVec, Computation, EagerVec, Format,
};

use crate::vecs::{
    Indexes,
    grouped::{ComputedVecBuilder, Source},
    indexes,
};

use super::{ComputedType, EagerVecBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: Option<EagerVec<Height, T>>,
    pub height_extra: EagerVecBuilder<Height, T>,
    pub dateindex: EagerVecBuilder<DateIndex, T>,
    pub weekindex: ComputedVecBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: EagerVecBuilder<DifficultyEpoch, T>,
    pub monthindex: ComputedVecBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: ComputedVecBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: ComputedVecBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: ComputedVecBuilder<YearIndex, T, DateIndex, YearIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: ComputedVecBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeight<T>
where
    T: ComputedType + Ord + From<f64> + 'static,
    f64: From<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        path: &Path,
        name: &str,
        source: Source<Height, T>,
        version: Version,
        format: Format,
        computation: Computation,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> color_eyre::Result<Self> {
        let height = source.is_compute().then(|| {
            EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format).unwrap()
        });

        let height_extra = EagerVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options.copy_self_extra(),
        )?;

        let dateindex = EagerVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options,
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            weekindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                None,
                &dateindex,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                None,
                &dateindex,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            )?,
            semesterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                None,
                &dateindex,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            )?,
            yearindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                None,
                &dateindex,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            )?,
            decadeindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                Computation::Lazy,
                None,
                &dateindex,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version + VERSION + Version::ZERO, format, options)?,
            height,
            height_extra,
            dateindex,
            difficultyepoch: EagerVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
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
        F: FnMut(&mut EagerVec<Height, T>, &Indexer, &indexes::Vecs, &Indexes, &Exit) -> Result<()>,
    {
        compute(
            self.height.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let height: Option<&EagerVec<Height, T>> = None;
        self.compute_rest(indexes, starting_indexes, exit, height)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        height_vec: Option<&impl AnyIterableVec<Height, T>>,
    ) -> color_eyre::Result<()> {
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
            self.height
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.height_extra.vecs(),
            self.dateindex.vecs(),
            self.weekindex.vecs(),
            self.difficultyepoch.vecs(),
            self.monthindex.vecs(),
            self.quarterindex.vecs(),
            self.semesterindex.vecs(),
            self.yearindex.vecs(),
            // self.halvingepoch.vecs(),
            self.decadeindex.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
