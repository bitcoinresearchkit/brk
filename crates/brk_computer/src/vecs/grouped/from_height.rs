use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, Result, Version,
    WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, AnyIterableVec, EagerVec, Format};

use crate::vecs::{Indexes, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: Option<EagerVec<Height, T>>,
    pub height_extra: ComputedVecBuilder<Height, T>,
    pub dateindex: ComputedVecBuilder<DateIndex, T>,
    pub weekindex: ComputedVecBuilder<WeekIndex, T>,
    pub difficultyepoch: ComputedVecBuilder<DifficultyEpoch, T>,
    pub monthindex: ComputedVecBuilder<MonthIndex, T>,
    pub quarterindex: ComputedVecBuilder<QuarterIndex, T>,
    pub yearindex: ComputedVecBuilder<YearIndex, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: ComputedVecBuilder<DecadeIndex, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeight<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        compute_source: bool,
        version: Version,
        format: Format,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let height = compute_source.then(|| {
            EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format).unwrap()
        });

        let height_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options.copy_self_extra(),
        )?;

        let dateindex = ComputedVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options,
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            dateindex,
            weekindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            difficultyepoch: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            yearindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version + VERSION + Version::ZERO, format, options)?,
            decadeindex: ComputedVecBuilder::forced_import(
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
        height: Option<&impl AnyIterableVec<Height, T>>,
    ) -> color_eyre::Result<()> {
        if let Some(height) = height {
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

        self.weekindex.from_aligned(
            starting_indexes.weekindex,
            &self.dateindex,
            &indexes.weekindex_to_first_dateindex,
            &indexes.weekindex_to_dateindex_count,
            exit,
        )?;

        self.monthindex.from_aligned(
            starting_indexes.monthindex,
            &self.dateindex,
            &indexes.monthindex_to_first_dateindex,
            &indexes.monthindex_to_dateindex_count,
            exit,
        )?;

        self.quarterindex.from_aligned(
            starting_indexes.quarterindex,
            &self.monthindex,
            &indexes.quarterindex_to_first_monthindex,
            &indexes.quarterindex_to_monthindex_count,
            exit,
        )?;

        self.yearindex.from_aligned(
            starting_indexes.yearindex,
            &self.monthindex,
            &indexes.yearindex_to_first_monthindex,
            &indexes.yearindex_to_monthindex_count,
            exit,
        )?;

        self.decadeindex.from_aligned(
            starting_indexes.decadeindex,
            &self.yearindex,
            &indexes.decadeindex_to_first_yearindex,
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
            self.yearindex.vecs(),
            // self.halvingepoch.vecs(),
            self.decadeindex.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
