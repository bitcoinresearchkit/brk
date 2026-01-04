use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, MonthIndex, QuarterIndex,
    Sats, SemesterIndex, TreeNode, TxIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{
    AnyExportableVec, AnyVec, CollectableVec, Database, EagerVec, Exit, GenericStoredVec,
    ImportableVec, IterableCloneableVec, PcoVec, TypedVecIterator, VecIndex,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{LazyVecsBuilder, Source},
    price,
    utils::OptionExt,
};

use crate::internal::{ComputedVecValue, EagerVecsBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub txindex: Option<Box<EagerVec<PcoVec<TxIndex, T>>>>,
    pub height: EagerVecsBuilder<Height, T>,
    pub dateindex: EagerVecsBuilder<DateIndex, T>,
    pub weekindex: LazyVecsBuilder<WeekIndex, T, DateIndex, WeekIndex>,
    pub difficultyepoch: LazyVecsBuilder<DifficultyEpoch, T, Height, DifficultyEpoch>,
    pub monthindex: LazyVecsBuilder<MonthIndex, T, DateIndex, MonthIndex>,
    pub quarterindex: LazyVecsBuilder<QuarterIndex, T, DateIndex, QuarterIndex>,
    pub semesterindex: LazyVecsBuilder<SemesterIndex, T, DateIndex, SemesterIndex>,
    pub yearindex: LazyVecsBuilder<YearIndex, T, DateIndex, YearIndex>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: LazyVecsBuilder<DecadeIndex, T, DateIndex, DecadeIndex>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromTxindex<T>
where
    T: ComputedVecValue + Ord + From<f64> + JsonSchema + 'static,
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
        let txindex = source
            .is_compute()
            .then(|| Box::new(EagerVec::forced_import(db, name, version + VERSION).unwrap()));

        let height = EagerVecsBuilder::forced_import(db, name, version + VERSION, options)?;

        let options = options.remove_percentiles();

        let dateindex = EagerVecsBuilder::forced_import(db, name, version + VERSION, options)?;

        Ok(Self {
            weekindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.weekindex_to_weekindex.boxed_clone(),
                options.into(),
            ),
            difficultyepoch: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &height,
                indexes
                    .block
                    .difficultyepoch_to_difficultyepoch
                    .boxed_clone(),
                options.into(),
            ),
            monthindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.monthindex_to_monthindex.boxed_clone(),
                options.into(),
            ),
            quarterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
                options.into(),
            ),
            semesterindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
                options.into(),
            ),
            yearindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.yearindex_to_yearindex.boxed_clone(),
                options.into(),
            ),
            decadeindex: LazyVecsBuilder::forced_import(
                name,
                version + VERSION,
                None,
                &dateindex,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
                options.into(),
            ),

            txindex,
            height,
            dateindex,
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION , format, options)?,
        })
    }

    // pub fn compute_all<F>(
    //     &mut self,
    //     indexer: &Indexer,
    //     indexes: &indexes::Vecs,
    //     starting_indexes: &ComputeIndexes,
    //     exit: &Exit,
    //     mut compute: F,
    // ) -> Result<()>
    // where
    //     F: FnMut(
    //         &mut EagerVec<PcoVec<TxIndex, T>>,
    //         &Indexer,
    //         &indexes::Vecs,
    //         &Indexes,
    //         &Exit,
    //     ) -> Result<()>,
    // {
    //     compute(
    //         self.txindex.um(),
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
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        txindex: Option<&impl CollectableVec<TxIndex, T>>,
    ) -> Result<()> {
        if let Some(txindex) = txindex {
            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs.tx.height_to_first_txindex,
                &indexes.block.height_to_txindex_count,
                exit,
            )?;
        } else {
            let txindex = self.txindex.u().as_ref();

            self.height.compute(
                starting_indexes.height,
                txindex,
                &indexer.vecs.tx.height_to_first_txindex,
                &indexes.block.height_to_txindex_count,
                exit,
            )?;
        }

        self.compute_after_height(indexes, starting_indexes, exit)
    }

    fn compute_after_height(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.dateindex.from_aligned(
            starting_indexes.dateindex,
            &self.height,
            &indexes.time.dateindex_to_first_height,
            &indexes.time.dateindex_to_height_count,
            exit,
        )?;

        Ok(())
    }
}

impl ComputedVecsFromTxindex<Bitcoin> {
    pub fn compute_rest_from_sats(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        sats: &ComputedVecsFromTxindex<Sats>,
        txindex: Option<&impl CollectableVec<TxIndex, Bitcoin>>,
    ) -> Result<()> {
        let txindex_version = if let Some(txindex) = txindex {
            txindex.version()
        } else {
            self.txindex.u().as_ref().version()
        };

        self.height
            .validate_computed_version_or_reset(txindex_version)?;

        let starting_index = self.height.starting_index(starting_indexes.height);

        // Create iterators once before the loop to avoid repeated iterator creation
        let mut first_iter = sats.height.first.as_ref().map(|v| v.into_iter());
        let mut average_iter = sats.height.average.as_ref().map(|v| v.into_iter());
        let mut sum_iter = sats.height.sum.as_ref().map(|v| v.into_iter());
        let mut max_iter = sats.height.max.as_ref().map(|v| v.into_iter());
        let mut pct90_iter = sats.height.pct90.as_ref().map(|v| v.into_iter());
        let mut pct75_iter = sats.height.pct75.as_ref().map(|v| v.into_iter());
        let mut median_iter = sats.height.median.as_ref().map(|v| v.into_iter());
        let mut pct25_iter = sats.height.pct25.as_ref().map(|v| v.into_iter());
        let mut pct10_iter = sats.height.pct10.as_ref().map(|v| v.into_iter());
        let mut min_iter = sats.height.min.as_ref().map(|v| v.into_iter());
        let mut last_iter = sats.height.last.as_ref().map(|v| v.into_iter());
        let mut cumulative_iter = sats.height.cumulative.as_ref().map(|v| v.into_iter());

        (starting_index.to_usize()..indexer.vecs.block.height_to_weight.len())
            .map(Height::from)
            .try_for_each(|height| -> Result<()> {
                if let Some(first) = self.height.first.as_mut() {
                    first
                        .truncate_push(height, Bitcoin::from(first_iter.um().get_unwrap(height)))?;
                }
                if let Some(average) = self.height.average.as_mut() {
                    average.truncate_push(
                        height,
                        Bitcoin::from(average_iter.um().get_unwrap(height)),
                    )?;
                }
                if let Some(sum) = self.height.sum.as_mut() {
                    sum.truncate_push(height, Bitcoin::from(sum_iter.um().get_unwrap(height)))?;
                }
                if let Some(max) = self.height.max.as_mut() {
                    max.truncate_push(height, Bitcoin::from(max_iter.um().get_unwrap(height)))?;
                }
                if let Some(pct90) = self.height.pct90.as_mut() {
                    pct90
                        .truncate_push(height, Bitcoin::from(pct90_iter.um().get_unwrap(height)))?;
                }
                if let Some(pct75) = self.height.pct75.as_mut() {
                    pct75
                        .truncate_push(height, Bitcoin::from(pct75_iter.um().get_unwrap(height)))?;
                }
                if let Some(median) = self.height.median.as_mut() {
                    median.truncate_push(
                        height,
                        Bitcoin::from(median_iter.um().get_unwrap(height)),
                    )?;
                }
                if let Some(pct25) = self.height.pct25.as_mut() {
                    pct25
                        .truncate_push(height, Bitcoin::from(pct25_iter.um().get_unwrap(height)))?;
                }
                if let Some(pct10) = self.height.pct10.as_mut() {
                    pct10
                        .truncate_push(height, Bitcoin::from(pct10_iter.um().get_unwrap(height)))?;
                }
                if let Some(min) = self.height.min.as_mut() {
                    min.truncate_push(height, Bitcoin::from(min_iter.um().get_unwrap(height)))?;
                }
                if let Some(last) = self.height.last.as_mut() {
                    last.truncate_push(height, Bitcoin::from(last_iter.um().get_unwrap(height)))?;
                }
                if let Some(cumulative) = self.height.cumulative.as_mut() {
                    cumulative.truncate_push(
                        height,
                        Bitcoin::from(cumulative_iter.um().get_unwrap(height)),
                    )?;
                }
                Ok(())
            })?;

        self.height.write()?;

        self.compute_after_height(indexes, starting_indexes, exit)
    }
}

impl ComputedVecsFromTxindex<Dollars> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_from_bitcoin(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        bitcoin: &ComputedVecsFromTxindex<Bitcoin>,
        txindex: Option<&impl CollectableVec<TxIndex, Dollars>>,
        price: &price::Vecs,
    ) -> Result<()> {
        let txindex_version = if let Some(txindex) = txindex {
            txindex.version()
        } else {
            self.txindex.u().as_ref().version()
        };

        self.height
            .validate_computed_version_or_reset(txindex_version)?;

        let starting_index = self.height.starting_index(starting_indexes.height);

        let mut close_iter = price.usd.chainindexes_to_price_close.height.into_iter();

        // Create iterators once before the loop to avoid repeated iterator creation
        let mut first_iter = bitcoin.height.first.as_ref().map(|v| v.into_iter());
        let mut average_iter = bitcoin.height.average.as_ref().map(|v| v.into_iter());
        let mut sum_iter = bitcoin.height.sum.as_ref().map(|v| v.into_iter());
        let mut max_iter = bitcoin.height.max.as_ref().map(|v| v.into_iter());
        let mut pct90_iter = bitcoin.height.pct90.as_ref().map(|v| v.into_iter());
        let mut pct75_iter = bitcoin.height.pct75.as_ref().map(|v| v.into_iter());
        let mut median_iter = bitcoin.height.median.as_ref().map(|v| v.into_iter());
        let mut pct25_iter = bitcoin.height.pct25.as_ref().map(|v| v.into_iter());
        let mut pct10_iter = bitcoin.height.pct10.as_ref().map(|v| v.into_iter());
        let mut min_iter = bitcoin.height.min.as_ref().map(|v| v.into_iter());
        let mut last_iter = bitcoin.height.last.as_ref().map(|v| v.into_iter());
        let mut cumulative_iter = bitcoin.height.cumulative.as_ref().map(|v| v.into_iter());

        (starting_index.to_usize()..indexer.vecs.block.height_to_weight.len())
            .map(Height::from)
            .try_for_each(|height| -> Result<()> {
                let price = *close_iter.get_unwrap(height);

                if let Some(first) = self.height.first.as_mut() {
                    first.truncate_push(height, price * first_iter.um().get_unwrap(height))?;
                }
                if let Some(average) = self.height.average.as_mut() {
                    average.truncate_push(height, price * average_iter.um().get_unwrap(height))?;
                }
                if let Some(sum) = self.height.sum.as_mut() {
                    sum.truncate_push(height, price * sum_iter.um().get_unwrap(height))?;
                }
                if let Some(max) = self.height.max.as_mut() {
                    max.truncate_push(height, price * max_iter.um().get_unwrap(height))?;
                }
                if let Some(pct90) = self.height.pct90.as_mut() {
                    pct90.truncate_push(height, price * pct90_iter.um().get_unwrap(height))?;
                }
                if let Some(pct75) = self.height.pct75.as_mut() {
                    pct75.truncate_push(height, price * pct75_iter.um().get_unwrap(height))?;
                }
                if let Some(median) = self.height.median.as_mut() {
                    median.truncate_push(height, price * median_iter.um().get_unwrap(height))?;
                }
                if let Some(pct25) = self.height.pct25.as_mut() {
                    pct25.truncate_push(height, price * pct25_iter.um().get_unwrap(height))?;
                }
                if let Some(pct10) = self.height.pct10.as_mut() {
                    pct10.truncate_push(height, price * pct10_iter.um().get_unwrap(height))?;
                }
                if let Some(min) = self.height.min.as_mut() {
                    min.truncate_push(height, price * min_iter.um().get_unwrap(height))?;
                }
                if let Some(last) = self.height.last.as_mut() {
                    last.truncate_push(height, price * last_iter.um().get_unwrap(height))?;
                }
                if let Some(cumulative) = self.height.cumulative.as_mut() {
                    cumulative
                        .truncate_push(height, price * cumulative_iter.um().get_unwrap(height))?;
                }
                Ok(())
            })?;

        self.height.write()?;

        self.compute_after_height(indexes, starting_indexes, exit)
    }
}

impl<T> Traversable for ComputedVecsFromTxindex<T>
where
    T: ComputedVecValue + JsonSchema,
{
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                self.txindex
                    .as_ref()
                    .map(|nested| ("txindex".to_string(), nested.to_tree_node())),
                Some(("height".to_string(), self.height.to_tree_node())),
                Some(("dateindex".to_string(), self.dateindex.to_tree_node())),
                Some(("weekindex".to_string(), self.weekindex.to_tree_node())),
                Some((
                    "difficultyepoch".to_string(),
                    self.difficultyepoch.to_tree_node(),
                )),
                Some(("monthindex".to_string(), self.monthindex.to_tree_node())),
                Some(("quarterindex".to_string(), self.quarterindex.to_tree_node())),
                Some((
                    "semesterindex".to_string(),
                    self.semesterindex.to_tree_node(),
                )),
                Some(("yearindex".to_string(), self.yearindex.to_tree_node())),
                Some(("decadeindex".to_string(), self.decadeindex.to_tree_node())),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .merge_branches()
        .unwrap()
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        let mut regular_iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(self.height.iter_any_exportable());
        regular_iter = Box::new(regular_iter.chain(self.dateindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_exportable()));
        if let Some(ref x) = self.txindex {
            regular_iter = Box::new(regular_iter.chain(x.iter_any_exportable()));
        }
        regular_iter
    }
}
