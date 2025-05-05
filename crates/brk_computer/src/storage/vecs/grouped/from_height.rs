use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyIterableVec, AnyVec, Compressed, EagerVec, Result, StoredVec, Version};

use crate::storage::{Indexes, indexes};

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
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let version = VERSION + version;

        let height = compute_source.then(|| {
            EagerVec::forced_import(&path.join(format!("height_to_{name}")), version, compressed)
                .unwrap()
        });

        let height_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version,
            compressed,
            options.copy_self_extra(),
        )?;

        let dateindex =
            ComputedVecBuilder::forced_import(path, name, version, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            dateindex,
            weekindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            difficultyepoch: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            yearindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version, compressed, options)?,
            decadeindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
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

        self.compute_rest(indexes, starting_indexes, exit, None)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&impl AnyIterableVec<Height, T>>,
    ) -> color_eyre::Result<()> {
        let height = height.unwrap_or_else(|| self.height.as_ref().unwrap().iter_vec());

        self.height_extra
            .extend(starting_indexes.height, height, exit)?;

        self.dateindex.compute(
            starting_indexes.dateindex,
            height,
            indexes.dateindex_to_first_height.iter_vec(),
            indexes.dateindex_to_height_count.iter_vec(),
            exit,
        )?;

        self.weekindex.from_aligned(
            starting_indexes.weekindex,
            &self.dateindex,
            indexes.weekindex_to_first_dateindex.iter_vec(),
            indexes.weekindex_to_dateindex_count.iter_vec(),
            exit,
        )?;

        self.monthindex.from_aligned(
            starting_indexes.monthindex,
            &self.dateindex,
            indexes.monthindex_to_first_dateindex.iter_vec(),
            indexes.monthindex_to_dateindex_count.iter_vec(),
            exit,
        )?;

        self.quarterindex.from_aligned(
            starting_indexes.quarterindex,
            &self.monthindex,
            indexes.quarterindex_to_first_monthindex.iter_vec(),
            indexes.quarterindex_to_monthindex_count.iter_vec(),
            exit,
        )?;

        self.yearindex.from_aligned(
            starting_indexes.yearindex,
            &self.monthindex,
            indexes.yearindex_to_first_monthindex.iter_vec(),
            indexes.yearindex_to_monthindex_count.iter_vec(),
            exit,
        )?;

        self.decadeindex.from_aligned(
            starting_indexes.decadeindex,
            &self.yearindex,
            indexes.decadeindex_to_first_yearindex.iter_vec(),
            indexes.decadeindex_to_yearindex_count.iter_vec(),
            exit,
        )?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            height,
            indexes.difficultyepoch_to_first_height.iter_vec(),
            indexes.difficultyepoch_to_height_count.iter_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyVec> {
        [
            self.height.as_ref().map_or(vec![], |v| vec![v.any_vec()]),
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
        .concat()
    }
}
