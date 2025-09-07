use allocative::Allocative;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{
    Bitcoin, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, MonthIndex, QuarterIndex,
    Sats, SemesterIndex, TxIndex, Version, WeekIndex, YearIndex,
};
use vecdb::{
    AnyCloneableIterableVec, AnyCollectableVec, AnyVec, CollectableVec, Database, EagerVec, Exit,
    GenericStoredVec, StoredIndex, VecIterator,
};

use crate::{
    Indexes,
    grouped::{LazyVecBuilder, Source},
    indexes, price,
};

use super::{ComputedType, EagerVecBuilder, VecBuilderOptions};

#[derive(Clone, Allocative)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub txindex: Option<Box<EagerVec<TxIndex, T>>>,
    pub height: EagerVecBuilder<Height, T>,
    pub dateindex: EagerVecBuilder<DateIndex, T>,
    pub weekindex: LazyVecBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: EagerVecBuilder<DifficultyEpoch, T>,
    pub monthindex: LazyVecBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecBuilder<YearIndex, T, DateIndex, YearIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: LazyVecBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromTxindex<T>
where
    T: ComputedType + Ord + From<f64> + 'static,
    f64: From<T>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<TxIndex, T>,
        version: Version,
        indexes: &indexes::Vecs,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let txindex = source.is_compute().then(|| {
            Box::new(
                EagerVec::forced_import_compressed(db, name, version + VERSION + Version::ZERO)
                    .unwrap(),
            )
        });

        let height = EagerVecBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options,
        )?;

        let options = options.remove_percentiles();

        let dateindex = EagerVecBuilder::forced_import_compressed(
            db,
            name,
            version + VERSION + Version::ZERO,
            options,
        )?;

        Ok(Self {
            weekindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecBuilder::forced_import(
                name,
                version + VERSION + Version::ZERO,
                None,
                &dateindex,
                indexes.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),

            txindex,
            height,
            dateindex,
            difficultyepoch: EagerVecBuilder::forced_import_compressed(
                db,
                name,
                version + VERSION + Version::ZERO,
                options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
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
    // ) -> Result<()>
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
            .validate_computed_version_or_reset(txindex_version)?;

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
                if let Some(_90p) = self.height.p90.as_mut() {
                    _90p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_p90()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_75p) = self.height.p75.as_mut() {
                    _75p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_p75()
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
                if let Some(_25p) = self.height.p25.as_mut() {
                    _25p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_p25()
                                .into_iter()
                                .unwrap_get_inner(height),
                        ),
                        exit,
                    )?;
                }
                if let Some(_10p) = self.height.p10.as_mut() {
                    _10p.forced_push_at(
                        height,
                        Bitcoin::from(
                            sats.height
                                .unwrap_p10()
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
        price: &price::Vecs,
    ) -> Result<()> {
        let txindex_version = if let Some(txindex) = txindex {
            txindex.version()
        } else {
            self.txindex.as_ref().unwrap().as_ref().version()
        };

        self.height
            .validate_computed_version_or_reset(txindex_version)?;

        let starting_index = self.height.starting_index(starting_indexes.height);

        let mut close_iter = price.chainindexes_to_price_close.height.into_iter();

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
                if let Some(_90p) = self.height.p90.as_mut() {
                    _90p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_p90()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_75p) = self.height.p75.as_mut() {
                    _75p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_p75()
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
                if let Some(_25p) = self.height.p25.as_mut() {
                    _25p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_p25()
                                .into_iter()
                                .unwrap_get_inner(height),
                        exit,
                    )?;
                }
                if let Some(_10p) = self.height.p10.as_mut() {
                    _10p.forced_push_at(
                        height,
                        price
                            * bitcoin
                                .height
                                .unwrap_p10()
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
