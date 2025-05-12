use std::path::Path;

use brk_core::{DateIndex, DecadeIndex, MonthIndex, QuarterIndex, WeekIndex, YearIndex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, EagerVec, Result, Version};

use crate::storage::{Indexes, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromDateIndex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: EagerVec<DateIndex, T>,
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
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let version = VERSION + version;

        let dateindex_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version,
            compressed,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            dateindex: EagerVec::forced_import(
                &path.join(format!("dateindex_to_{name}")),
                version,
                compressed,
            )?,
            dateindex_extra,
            weekindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            monthindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            yearindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            decadeindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
        })
    }

    pub fn compute<F>(
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
            &mut self.dateindex,
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        self.compute_rest(indexes, starting_indexes, exit)
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.dateindex_extra
            .extend(starting_indexes.dateindex, &self.dateindex, exit)?;

        self.weekindex.compute(
            starting_indexes.weekindex,
            &self.dateindex,
            &indexes.weekindex_to_first_dateindex,
            &indexes.weekindex_to_dateindex_count,
            exit,
        )?;

        self.monthindex.compute(
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
            vec![&self.dateindex as &dyn AnyCollectableVec],
            self.dateindex_extra.vecs(),
            self.weekindex.vecs(),
            self.monthindex.vecs(),
            self.quarterindex.vecs(),
            self.yearindex.vecs(),
            self.decadeindex.vecs(),
        ]
        .concat()
    }
}
