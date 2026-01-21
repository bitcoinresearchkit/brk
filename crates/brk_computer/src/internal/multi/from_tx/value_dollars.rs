//! Dollars from TxIndex with lazy height stats and stored dateindex.
//!
//! Height-level USD stats (min/max/avg/sum/percentiles) are lazy: `sats_stat * price`.
//! Height cumulative and dateindex stats are stored since they require aggregation
//! across heights with varying prices.

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, Close, DateIndex, DifficultyEpoch, Dollars, Height, Sats, TxIndex, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, IterableBoxedVec, IterableCloneableVec, LazyVecFrom3,
};

use crate::{
    indexes,
    internal::{
        CumulativeVec, Full, LazyBinaryTransformFull, LazyDateDerivedFull, LazyFull,
        SatsTimesClosePrice, Stats,
    },
    ComputeIndexes,
};

/// Lazy dollars at TxIndex: `sats * price[height]`
pub type LazyDollarsTxIndex =
    LazyVecFrom3<TxIndex, Dollars, TxIndex, Sats, TxIndex, Height, Height, Close<Dollars>>;

/// Lazy dollars height stats: `sats_height_stat * price`
pub type LazyDollarsHeightFull = LazyBinaryTransformFull<Height, Dollars, Sats, Close<Dollars>>;

/// Dollars with lazy txindex and height fields, stored dateindex.
///
/// Height-level stats (except cumulative) are lazy: `sats * price[height]`.
/// Cumulative at height level is stored since it requires summing historical values.
/// DateIndex stats are stored since they aggregate across heights with varying prices.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ValueDollarsFromTxFull {
    #[traversable(skip)]
    pub txindex: LazyDollarsTxIndex,
    #[traversable(flatten)]
    pub height: LazyDollarsHeightFull,
    pub height_cumulative: CumulativeVec<Height, Dollars>,
    pub difficultyepoch: LazyFull<DifficultyEpoch, Dollars, Height, DifficultyEpoch>,
    pub dateindex: Stats<DateIndex, Dollars>,
    #[deref]
    #[deref_mut]
    pub dates: LazyDateDerivedFull<Dollars>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height change

impl ValueDollarsFromTxFull {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        sats_height: &Full<Height, Sats>,
        height_to_price: IterableBoxedVec<Height, Close<Dollars>>,
        sats_txindex: IterableBoxedVec<TxIndex, Sats>,
        txindex_to_height: IterableBoxedVec<TxIndex, Height>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let txindex = create_lazy_txindex(
            name,
            v,
            sats_txindex,
            txindex_to_height,
            height_to_price.clone(),
        );

        // Lazy height stats: sats_stat * price
        let height = LazyBinaryTransformFull::from_full_and_source::<SatsTimesClosePrice>(
            name,
            v,
            sats_height,
            height_to_price.clone(),
        );

        // Stored cumulative - must be computed by summing historical sum*price
        let height_cumulative = CumulativeVec(EagerVec::forced_import(
            db,
            &format!("{name}_cumulative"),
            v,
        )?);

        let dateindex = Stats::forced_import(db, name, v)?;

        let difficultyepoch =
            LazyFull::<DifficultyEpoch, Dollars, Height, DifficultyEpoch>::from_stats_aggregate(
                name,
                v,
                height.boxed_average(),
                height.boxed_min(),
                height.boxed_max(),
                height.boxed_sum(),
                height_cumulative.0.boxed_clone(),
                indexes.difficultyepoch.identity.boxed_clone(),
            );

        let dates = LazyDateDerivedFull::from_sources(
            name,
            v,
            dateindex.boxed_average(),
            dateindex.boxed_min(),
            dateindex.boxed_max(),
            dateindex.boxed_sum(),
            dateindex.boxed_cumulative(),
            indexes,
        );

        Ok(Self {
            txindex,
            height,
            height_cumulative,
            difficultyepoch,
            dateindex,
            dates,
        })
    }

    /// Compute stored fields (cumulative and dateindex) from lazy height stats.
    ///
    /// This is MUCH faster than the old approach since it only iterates heights,
    /// not all transactions per block.
    pub fn derive_from(
        &mut self,
        _indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Compute height cumulative by summing lazy height.sum values
        self.height_cumulative.0.compute_cumulative(
            starting_indexes.height,
            &self.height.sum,
            exit,
        )?;

        // Compute dateindex stats by aggregating lazy height stats
        self.dateindex.compute(
            starting_indexes.dateindex,
            &self.height.average,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
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
