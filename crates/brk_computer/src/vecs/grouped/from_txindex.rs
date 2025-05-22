use std::path::Path;

use brk_core::{
    Bitcoin, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, MonthIndex, QuarterIndex,
    Sats, TxIndex, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, CollectableVec, Compressed, EagerVec, Result, StoredIndex,
    VecIterator, Version,
};

use crate::vecs::{Indexes, fetched, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub txindex: Option<Box<EagerVec<TxIndex, T>>>,
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

        let txindex = compute_source
            .then(|| Box::new(EagerVec::forced_import(path, name, version, compressed).unwrap()));

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

    // #[allow(unused)]
    // pub fn compute_all<F>(
    //     &mut self,
    //     indexer: &Indexer,
    //     indexes: &indexes::Vecs,
    //     starting_indexes: &Indexes,
    //     exit: &Exit,
    //     mut compute: F,
    // ) -> color_eyre::Result<()>
    // where
    //     F: FnMut(
    //         &mut EagerVec<TxIndex, T>,
    //         &Indexer,
    //         &indexes::Vecs,
    //         &Indexes,
    //         &Exit,
    //     ) -> Result<()>,
    // {
    //     compute(
    //         self.txindex.as_mut().unwrap(),
    //         indexer,
    //         indexes,
    //         starting_indexes,
    //         exit,
    //     )?;

    //     let txindex: Option<&StoredVec<TxIndex, T>> = None;
    //     self.compute_rest(indexer, indexes, starting_indexes, exit, txindex)?;

    //     Ok(())
    // }

    pub fn compute_rest(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        txindex: Option<&impl CollectableVec<TxIndex, T>>,
    ) -> Result<()> {
        if let Some(txindex) = txindex {
            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs().height_to_first_txindex,
                &indexes.height_to_txindex_count,
                exit,
            )?;
        } else {
            let txindex = self.txindex.as_ref().unwrap().as_ref();

            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs().height_to_first_txindex,
                &indexes.height_to_txindex_count,
                exit,
            )?;
        }

        self.compute_after_height(indexes, starting_indexes, exit)
    }

    fn compute_after_height(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
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
                .map_or(vec![], |v| vec![v.as_ref() as &dyn AnyCollectableVec]),
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
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

impl ComputedVecsFromTxindex<Bitcoin> {
    pub fn compute_rest_from_sats(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        sats: &ComputedVecsFromTxindex<Sats>,
        txindex: Option<&impl CollectableVec<TxIndex, Bitcoin>>,
    ) -> Result<()> {
        let txindex_version = if let Some(txindex) = txindex {
            txindex.version()
        } else {
            self.txindex.as_ref().unwrap().as_ref().version()
        };

        self.height
            .validate_computed_version_or_reset_file(txindex_version)?;

        let starting_index = self.height.starting_index(starting_indexes.height);

        (starting_index.unwrap_to_usize()..indexer.vecs().height_to_weight.len())
            .map(Height::from)
            .try_for_each(|height| -> Result<()> {
                if let Some(first) = self.height.first.as_mut() {
                    first.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_first()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(average) = self.height.average.as_mut() {
                    average.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_average()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(sum) = self.height.sum.as_mut() {
                    sum.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_sum()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(max) = self.height.max.as_mut() {
                    max.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_max()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_90p) = self.height._90p.as_mut() {
                    _90p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_90p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_75p) = self.height._75p.as_mut() {
                    _75p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_75p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(median) = self.height.median.as_mut() {
                    median.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_median()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_25p) = self.height._25p.as_mut() {
                    _25p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_25p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_10p) = self.height._10p.as_mut() {
                    _10p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_10p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(min) = self.height.min.as_mut() {
                    min.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_min()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(last) = self.height.last.as_mut() {
                    last.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_last()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(total) = self.height.total.as_mut() {
                    total.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_total()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                Ok(())
            })?;

        self.height.safe_flush(exit)?;

        self.compute_after_height(indexes, starting_indexes, exit)
    }
}

impl ComputedVecsFromTxindex<Dollars> {
    pub fn compute_rest_from_bitcoin(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        bitcoin: &ComputedVecsFromTxindex<Bitcoin>,
        txindex: Option<&impl CollectableVec<TxIndex, Dollars>>,
        fetched: &fetched::Vecs,
    ) -> Result<()> {
        let txindex_version = if let Some(txindex) = txindex {
            txindex.version()
        } else {
            self.txindex.as_ref().unwrap().as_ref().version()
        };

        self.height
            .validate_computed_version_or_reset_file(txindex_version)?;

        let starting_index = self.height.starting_index(starting_indexes.height);

        let mut close_iter = fetched.chainindexes_to_close.height.into_iter();

        (starting_index.unwrap_to_usize()..indexer.vecs().height_to_weight.len())
            .map(Height::from)
            .try_for_each(|height| -> Result<()> {
                let price = *close_iter.unwrap_get_inner(height);

                if let Some(first) = self.height.first.as_mut() {
                    first.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_first()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(average) = self.height.average.as_mut() {
                    average.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_average()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(sum) = self.height.sum.as_mut() {
                    sum.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_sum()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(max) = self.height.max.as_mut() {
                    max.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_max()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_90p) = self.height._90p.as_mut() {
                    _90p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_90p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_75p) = self.height._75p.as_mut() {
                    _75p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_75p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(median) = self.height.median.as_mut() {
                    median.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_median()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_25p) = self.height._25p.as_mut() {
                    _25p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_25p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_10p) = self.height._10p.as_mut() {
                    _10p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_10p()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(min) = self.height.min.as_mut() {
                    min.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_min()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(last) = self.height.last.as_mut() {
                    last.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_last()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(total) = self.height.total.as_mut() {
                    total.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_total()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                Ok(())
            })?;

        self.height.safe_flush(exit)?;

        self.compute_after_height(indexes, starting_indexes, exit)
    }
}
