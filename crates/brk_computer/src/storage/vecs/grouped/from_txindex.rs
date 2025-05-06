use std::path::Path;

use brk_core::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, TxIndex, WeekIndex,
    YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, CollectableVec, Compressed, EagerVec, Result, StoredVec, Version,
};

use crate::storage::{Indexes, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub txindex: Option<EagerVec<TxIndex, T>>,
    pub height: ComputedVecBuilder<Height, T>,
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

impl<T> ComputedVecsFromTxindex<T>
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

        let txindex = compute_source.then(|| {
            EagerVec::forced_import(
                &path.join(format!("txindex_to_{name}")),
                version,
                compressed,
            )
            .unwrap()
        });

        let height = ComputedVecBuilder::forced_import(path, name, version, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            txindex,
            height,
            dateindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
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
        F: FnMut(
            &mut EagerVec<TxIndex, T>,
            &Indexer,
            &indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.txindex.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        let txindex: Option<&StoredVec<TxIndex, T>> = None;
        self.compute_rest(indexer, indexes, starting_indexes, exit, txindex)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        txindex: Option<&impl CollectableVec<TxIndex, T>>,
    ) -> color_eyre::Result<()> {
        if let Some(txindex) = txindex {
            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs().height_to_first_txindex,
                &indexes.height_to_txindex_count,
                exit,
            )?;
        } else {
            let txindex = self.txindex.as_ref().unwrap();

            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs().height_to_first_txindex,
                &indexes.height_to_txindex_count,
                exit,
            )?;
        }

        self.dateindex.from_aligned(
            starting_indexes.dateindex,
            &self.height,
            &indexes.dateindex_to_first_height,
            &indexes.dateindex_to_height_count,
            exit,
        )?;

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

        self.difficultyepoch.from_aligned(
            starting_indexes.difficultyepoch,
            &self.height,
            &indexes.difficultyepoch_to_first_height,
            &indexes.difficultyepoch_to_height_count,
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.txindex
                .as_ref()
                .map_or(vec![], |v| vec![v as &dyn AnyCollectableVec]),
            self.height.vecs(),
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
