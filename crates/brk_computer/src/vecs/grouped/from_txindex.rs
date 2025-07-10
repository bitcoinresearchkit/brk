use std::path::Path;

use brk_core::{
    Bitcoin, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, MonthIndex, QuarterIndex,
    Result, Sats, SemesterIndex, TxIndex, Version, WeekIndex, YearIndex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{
    AnyCollectableVec, AnyVec, CollectableVec, Computation, EagerVec, Format, StoredIndex,
    VecIterator,
};

use crate::vecs::{
    Indexes, fetched,
    grouped::{ComputedVecBuilder, Source},
    indexes,
};

use super::{ComputedType, EagerVecBuilder, EagerVecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub txindex: Option<Box<EagerVec<TxIndex, T>>>,
    pub height: EagerVecBuilder<Height, T>,
    pub dateindex: EagerVecBuilder<DateIndex, T>,
    pub weekindex: ComputedVecBuilder<WeekIndex, T, DateIndex>,
    pub difficultyepoch: EagerVecBuilder<DifficultyEpoch, T>,
    pub monthindex: ComputedVecBuilder<MonthIndex, T, DateIndex>,
    pub quarterindex: ComputedVecBuilder<QuarterIndex, T, DateIndex>,
    pub semesterindex: ComputedVecBuilder<SemesterIndex, T, DateIndex>,
    pub yearindex: ComputedVecBuilder<YearIndex, T, DateIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: ComputedVecBuilder<DecadeIndex, T, DateIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromTxindex<T>
where
    T: ComputedType + Ord + From<f64> + 'static,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        source: Source<TxIndex, T>,
        version: Version,
        format: Format,
        computation: Computation,
        options: EagerVecBuilderOptions,
    ) -> color_eyre::Result<Self> {
        let txindex = source.is_compute().then(|| {
            Box::new(
                EagerVec::forced_import(path, name, version + VERSION + Version::ZERO, format)
                    .unwrap(),
            )
        });

        let height = EagerVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options,
        )?;

        let options = options.remove_percentiles();

        let dateindex = EagerVecBuilder::forced_import(
            path,
            name,
            version + VERSION + Version::ZERO,
            format,
            options,
        )?;

        Ok(Self {
            weekindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,
            semesterindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,
            yearindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,
            decadeindex: ComputedVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                computation,
                None,
                &dateindex,
                options.into(),
            )?,

            txindex,
            height,
            dateindex,
            difficultyepoch: EagerVecBuilder::forced_import(
                path,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version + VERSION + Version::ZERO, format, options)?,
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
                &indexer.vecs.height_to_first_txindex,
                &indexes.height_to_txindex_count,
                exit,
            )?;
        } else {
            let txindex = self.txindex.as_ref().unwrap().as_ref();

            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs.height_to_first_txindex,
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

        self.weekindex.compute_if_necessary(
            starting_indexes.weekindex,
            &indexes.weekindex_to_first_dateindex,
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

        (starting_index.unwrap_to_usize()..indexer.vecs.height_to_weight.len())
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
                if let Some(cumulative) = self.height.cumulative.as_mut() {
                    cumulative.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_cumulative()
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
    #[allow(clippy::too_many_arguments)]
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

        (starting_index.unwrap_to_usize()..indexer.vecs.height_to_weight.len())
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
                if let Some(cumulative) = self.height.cumulative.as_mut() {
                    cumulative.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_cumulative()
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
