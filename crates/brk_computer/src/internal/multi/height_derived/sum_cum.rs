//! ComputedHeightDerivedSumCum - height cumulative (stored) + lazy time periods + epochs.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, Version, Week1, Year1, Year10,
};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedVecValue, CumulativeVec, LazySumCum, NumericValue},
    ComputeIndexes,
};

#[derive(Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedSumCum<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "cumulative")]
    pub height_cumulative: CumulativeVec<Height, T, M>,
    pub minute1: LazySumCum<Minute1, T, Height, Height>,
    pub minute5: LazySumCum<Minute5, T, Height, Height>,
    pub minute10: LazySumCum<Minute10, T, Height, Height>,
    pub minute30: LazySumCum<Minute30, T, Height, Height>,
    pub hour1: LazySumCum<Hour1, T, Height, Height>,
    pub hour4: LazySumCum<Hour4, T, Height, Height>,
    pub hour12: LazySumCum<Hour12, T, Height, Height>,
    pub day1: LazySumCum<Day1, T, Height, Height>,
    pub day3: LazySumCum<Day3, T, Height, Height>,
    pub week1: LazySumCum<Week1, T, Height, Height>,
    pub month1: LazySumCum<Month1, T, Height, Height>,
    pub month3: LazySumCum<Month3, T, Height, Height>,
    pub month6: LazySumCum<Month6, T, Height, Height>,
    pub year1: LazySumCum<Year1, T, Height, Height>,
    pub year10: LazySumCum<Year10, T, Height, Height>,
    pub halvingepoch: LazySumCum<HalvingEpoch, T, Height, HalvingEpoch>,
    pub difficultyepoch: LazySumCum<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedSumCum<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        height_source: ReadableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height_cumulative = CumulativeVec::forced_import(db, name, v)?;

        macro_rules! period {
            ($idx:ident) => {
                LazySumCum::from_height_sources_sum_raw(
                    name,
                    v,
                    height_source.clone(),
                    height_cumulative.read_only_boxed_clone(),
                    indexes.$idx.first_height.read_only_boxed_clone(),
                )
            };
        }

        macro_rules! epoch {
            ($idx:ident) => {
                LazySumCum::from_sources_sum_raw(
                    name,
                    v,
                    height_source.clone(),
                    height_cumulative.read_only_boxed_clone(),
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

    pub(crate) fn derive_from(
        &mut self,
        starting_indexes: &ComputeIndexes,
        height_source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.height_cumulative
            .0
            .compute_cumulative(starting_indexes.height, height_source, exit)?;
        Ok(())
    }
}
