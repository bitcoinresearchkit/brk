//! Dollars from TxIndex with lazy height stats and stored day1.
//!
//! Height-level USD stats (min/max/avg/sum/percentiles) are lazy: `sats_stat * price`.
//! Height cumulative and day1 stats are stored since they require aggregation
//! across heights with varying prices.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    Bitcoin, Day1, Day3, DifficultyEpoch, Dollars, HalvingEpoch, Height, Hour1, Hour4, Hour12,
    Minute1, Minute5, Minute10, Minute30, Month1, Month3, Month6, Sats, TxIndex, Version, Week1,
    Year1, Year10,
};
use vecdb::{
    Database, Exit, LazyVecFrom3, ReadableBoxedVec, ReadableCloneableVec, Rw, StorageMode,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{CumulativeVec, Full, LazyBinaryTransformFull, LazyFull, SatsTimesPrice},
};

/// Lazy dollars at TxIndex: `sats * price[height]`
pub type LazyDollarsTxIndex =
    LazyVecFrom3<TxIndex, Dollars, TxIndex, Sats, TxIndex, Height, Height, Dollars>;

/// Lazy dollars height stats: `sats_height_stat * price`
pub type LazyDollarsHeightFull = LazyBinaryTransformFull<Height, Dollars, Sats, Dollars>;

/// Dollars with lazy txindex and height fields, stored day1.
///
/// Height-level stats (except cumulative) are lazy: `sats * price[height]`.
/// Cumulative at height level is stored since it requires summing historical values.
/// Day1 stats are stored since they aggregate across heights with varying prices.
#[derive(Traversable)]
#[traversable(merge)]
pub struct ValueDollarsFromTxFull<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub txindex: LazyDollarsTxIndex,
    #[traversable(flatten)]
    pub height: LazyDollarsHeightFull,
    #[traversable(rename = "cumulative")]
    pub height_cumulative: CumulativeVec<Height, Dollars, M>,
    pub minute1: LazyFull<Minute1, Dollars, Height, Height>,
    pub minute5: LazyFull<Minute5, Dollars, Height, Height>,
    pub minute10: LazyFull<Minute10, Dollars, Height, Height>,
    pub minute30: LazyFull<Minute30, Dollars, Height, Height>,
    pub hour1: LazyFull<Hour1, Dollars, Height, Height>,
    pub hour4: LazyFull<Hour4, Dollars, Height, Height>,
    pub hour12: LazyFull<Hour12, Dollars, Height, Height>,
    pub day1: LazyFull<Day1, Dollars, Height, Height>,
    pub day3: LazyFull<Day3, Dollars, Height, Height>,
    pub week1: LazyFull<Week1, Dollars, Height, Height>,
    pub month1: LazyFull<Month1, Dollars, Height, Height>,
    pub month3: LazyFull<Month3, Dollars, Height, Height>,
    pub month6: LazyFull<Month6, Dollars, Height, Height>,
    pub year1: LazyFull<Year1, Dollars, Height, Height>,
    pub year10: LazyFull<Year10, Dollars, Height, Height>,
    pub halvingepoch: LazyFull<HalvingEpoch, Dollars, Height, HalvingEpoch>,
    pub difficultyepoch: LazyFull<DifficultyEpoch, Dollars, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ONE; // Bumped for lazy height change

impl ValueDollarsFromTxFull {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        sats_height: &Full<Height, Sats>,
        height_to_price: ReadableBoxedVec<Height, Dollars>,
        sats_txindex: ReadableBoxedVec<TxIndex, Sats>,
        txindex_to_height: ReadableBoxedVec<TxIndex, Height>,
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
        let height = LazyBinaryTransformFull::from_full_and_source::<SatsTimesPrice>(
            name,
            v,
            sats_height,
            height_to_price.clone(),
        );

        // Stored cumulative - must be computed by summing historical sum*price
        let height_cumulative = CumulativeVec::forced_import(db, name, v)?;

        macro_rules! period {
            ($idx:ident) => {
                LazyFull::from_height_source(
                    name,
                    v,
                    height.boxed_sum(),
                    height_cumulative.read_only_boxed_clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazyFull::from_stats_aggregate(
                    name,
                    v,
                    height.boxed_average(),
                    height.boxed_min(),
                    height.boxed_max(),
                    height.boxed_sum(),
                    height_cumulative.read_only_boxed_clone(),
                    height.boxed_average(),
                    indexes.$idx.identity.read_only_boxed_clone(),
                )
            };
        }

        let minute1 = period!(minute1);
        let minute5 = period!(minute5);
        let minute10 = period!(minute10);
        let minute30 = period!(minute30);
        let hour1 = period!(hour1);
        let hour4 = period!(hour4);
        let hour12 = period!(hour12);
        let day1 = period!(day1);
        let day3 = period!(day3);
        let week1 = period!(week1);
        let month1 = period!(month1);
        let month3 = period!(month3);
        let month6 = period!(month6);
        let year1 = period!(year1);
        let year10 = period!(year10);
        let halvingepoch = epoch!(halvingepoch);
        let difficultyepoch = epoch!(difficultyepoch);

        Ok(Self {
            txindex,
            height,
            height_cumulative,
            minute1,
            minute5,
            minute10,
            minute30,
            hour1,
            hour4,
            hour12,
            day1,
            day3,
            week1,
            month1,
            month3,
            month6,
            year1,
            year10,
            halvingepoch,
            difficultyepoch,
        })
    }

    /// Compute stored fields (cumulative and day1) from lazy height stats.
    ///
    /// This is MUCH faster than the old approach since it only iterates heights,
    /// not all transactions per block.
    pub(crate) fn derive_from(
        &mut self,
        _indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Compute height cumulative by summing lazy height.sum values
        self.height_cumulative.0.compute_cumulative(
            starting_indexes.height,
            &self.height.sum,
            exit,
        )?;

        Ok(())
    }
}

fn create_lazy_txindex(
    name: &str,
    version: Version,
    sats_txindex: ReadableBoxedVec<TxIndex, Sats>,
    txindex_to_height: ReadableBoxedVec<TxIndex, Height>,
    height_to_price: ReadableBoxedVec<Height, Dollars>,
) -> LazyDollarsTxIndex {
    LazyVecFrom3::init(
        &format!("{name}_txindex"),
        version,
        sats_txindex,
        txindex_to_height,
        height_to_price,
        |_index, sats, _height, close| close * Bitcoin::from(sats),
    )
}
