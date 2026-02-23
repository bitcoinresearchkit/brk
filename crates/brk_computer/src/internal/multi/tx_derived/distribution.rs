//! TxDerivedDistribution - computes TxIndex data to height Distribution + lazy time periods + epochs.

use brk_error::Result;
use brk_indexer::Indexer;

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, TxIndex, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, Distribution, LazyDistribution, NumericValue},
};

#[derive(Traversable)]
#[traversable(merge)]
pub struct TxDerivedDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: Distribution<Height, T, M>,
    pub minute1: LazyDistribution<Minute1, T, Height, Height>,
    pub minute5: LazyDistribution<Minute5, T, Height, Height>,
    pub minute10: LazyDistribution<Minute10, T, Height, Height>,
    pub minute30: LazyDistribution<Minute30, T, Height, Height>,
    pub hour1: LazyDistribution<Hour1, T, Height, Height>,
    pub hour4: LazyDistribution<Hour4, T, Height, Height>,
    pub hour12: LazyDistribution<Hour12, T, Height, Height>,
    pub day1: LazyDistribution<Day1, T, Height, Height>,
    pub day3: LazyDistribution<Day3, T, Height, Height>,
    pub week1: LazyDistribution<Week1, T, Height, Height>,
    pub month1: LazyDistribution<Month1, T, Height, Height>,
    pub month3: LazyDistribution<Month3, T, Height, Height>,
    pub month6: LazyDistribution<Month6, T, Height, Height>,
    pub year1: LazyDistribution<Year1, T, Height, Height>,
    pub year10: LazyDistribution<Year10, T, Height, Height>,
    pub halvingepoch: LazyDistribution<HalvingEpoch, T, Height, HalvingEpoch>,
    pub difficultyepoch: LazyDistribution<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> TxDerivedDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height = Distribution::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        macro_rules! period {
            ($idx:ident) => {
                LazyDistribution::from_height_source(
                    name,
                    v,
                    height.boxed_average(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazyDistribution::from_source(
                    name,
                    v,
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
            height,
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

    pub(crate) fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl ReadableVec<TxIndex, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.derive_from_with_skip(indexer, indexes, starting_indexes, txindex_source, exit, 0)
    }

    /// Derive from source, skipping first N transactions per block from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl ReadableVec<TxIndex, T>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()> {
        self.height.compute_with_skip(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            skip_count,
        )?;

        Ok(())
    }
}
