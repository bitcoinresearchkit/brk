//! Dollars from TxIndex with lazy txindex and eager aggregates.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, Close, DateIndex, DifficultyEpoch, Dollars, Height, Sats, TxIndex, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, LazyVecFrom3};

use crate::{
    ComputeIndexes, indexes,
    internal::{DerivedDateFull, Full, LazyFull, Stats},
};

/// Lazy dollars at TxIndex: `sats * price[height]`
pub type LazyDollarsTxIndex =
    LazyVecFrom3<TxIndex, Dollars, TxIndex, Sats, TxIndex, Height, Height, Close<Dollars>>;

/// Dollars with lazy txindex field and eager height/dateindex aggregates.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct DollarsTxFull {
    #[traversable(skip)]
    pub txindex: LazyDollarsTxIndex,
    pub height: Full<Height, Dollars>,
    pub difficultyepoch: LazyFull<DifficultyEpoch, Dollars, Height, DifficultyEpoch>,
    pub dateindex: Stats<DateIndex, Dollars>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: DerivedDateFull<Dollars>,
}

const VERSION: Version = Version::ZERO;

impl DollarsTxFull {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        sats_txindex: IterableBoxedVec<TxIndex, Sats>,
        txindex_to_height: IterableBoxedVec<TxIndex, Height>,
        height_to_price: IterableBoxedVec<Height, Close<Dollars>>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let txindex =
            create_lazy_txindex(name, v, sats_txindex, txindex_to_height, height_to_price);
        let height = Full::forced_import(db, name, v)?;
        let dateindex = Stats::forced_import(db, name, v)?;

        let difficultyepoch =
            LazyFull::<DifficultyEpoch, Dollars, Height, DifficultyEpoch>::from_stats_aggregate(
                name,
                v,
                height.distribution.average.0.boxed_clone(),
                height.distribution.minmax.min.0.boxed_clone(),
                height.distribution.minmax.max.0.boxed_clone(),
                height.sum_cum.sum.0.boxed_clone(),
                height.sum_cum.cumulative.0.boxed_clone(),
                indexes
                    .block
                    .difficultyepoch_to_difficultyepoch
                    .boxed_clone(),
            );

        let dates = DerivedDateFull::from_sources(
            name,
            v,
            dateindex.average.0.boxed_clone(),
            dateindex.minmax.min.0.boxed_clone(),
            dateindex.minmax.max.0.boxed_clone(),
            dateindex.sum_cum.sum.0.boxed_clone(),
            dateindex.sum_cum.cumulative.0.boxed_clone(),
            indexes,
        );

        Ok(Self {
            txindex,
            height,
            difficultyepoch,
            dateindex,
            dates,
        })
    }

    pub fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height.compute(
            starting_indexes.height,
            &self.txindex,
            &indexer.vecs.tx.height_to_first_txindex,
            &indexes.block.height_to_txindex_count,
            exit,
        )?;

        self.dateindex.compute(
            starting_indexes.dateindex,
            &self.height.distribution.average.0,
            &indexes.time.dateindex_to_first_height,
            &indexes.time.dateindex_to_height_count,
            exit,
        )?;

        Ok(())
    }
}

fn create_lazy_txindex(
    name: &str,
    version: Version,
    sats_txindex: IterableBoxedVec<TxIndex, Sats>,
    txindex_to_height: IterableBoxedVec<TxIndex, Height>,
    height_to_price: IterableBoxedVec<Height, Close<Dollars>>,
) -> LazyDollarsTxIndex {
    LazyVecFrom3::init(
        &format!("{name}_txindex"),
        version,
        sats_txindex,
        txindex_to_height,
        height_to_price,
        |txindex, sats_iter, height_iter, price_iter| {
            sats_iter.get(txindex).and_then(|sats| {
                height_iter.get(txindex).and_then(|height| {
                    price_iter
                        .get(height)
                        .map(|close| *close * Bitcoin::from(sats))
                })
            })
        },
    )
}
