use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, Result, Version, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, AnyIterableVec, EagerVec, Format};

use crate::vecs::{Indexes, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromDateIndex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: Option<EagerVec<DateIndex, T>>,
    pub dateindex_extra: ComputedVecBuilder<DateIndex, T>,
    pub weekindex: ComputedVecBuilder<WeekIndex, T>,
    pub monthindex: ComputedVecBuilder<MonthIndex, T>,
    pub quarterindex: ComputedVecBuilder<QuarterIndex, T>,
    pub yearindex: ComputedVecBuilder<YearIndex, T>,
    pub decadeindex: ComputedVecBuilder<DecadeIndex, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromDateIndex<T>
where
    T: ComputedType,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        compute_source: bool,
        version: Version,
        format: Format,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let dateindex = compute_source.then(|| {
            EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format).unwrap()
        });

        let dateindex_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            dateindex,
            dateindex_extra,
            weekindex: ComputedVecBuilder::forced_import(
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

            self.weekindex.compute(
                starting_indexes.weekindex,
                dateindex,
                &indexes.weekindex_to_first_dateindex,
                &indexes.weekindex_to_dateindex_count,
                exit,
            )?;

            self.monthindex.compute(
                starting_indexes.monthindex,
                dateindex,
                &indexes.monthindex_to_first_dateindex,
                &indexes.monthindex_to_dateindex_count,
                exit,
            )?;
        } else {
            let dateindex = self.dateindex.as_ref().unwrap();

            self.dateindex_extra
                .extend(starting_indexes.dateindex, dateindex, exit)?;

            self.weekindex.compute(
                starting_indexes.weekindex,
                dateindex,
                &indexes.weekindex_to_first_dateindex,
                &indexes.weekindex_to_dateindex_count,
                exit,
            )?;

            self.monthindex.compute(
                starting_indexes.monthindex,
                dateindex,
                &indexes.monthindex_to_first_dateindex,
                &indexes.monthindex_to_dateindex_count,
                exit,
            )?;
        }

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
            self.dateindex
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.dateindex_extra.vecs(),
            self.weekindex.vecs(),
            self.monthindex.vecs(),
            self.quarterindex.vecs(),
            self.yearindex.vecs(),
            self.decadeindex.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
