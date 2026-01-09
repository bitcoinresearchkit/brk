//! OHLC period groupings for all time/chain periods.

use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{BytesVec, BytesVecValue, EagerVec, Formattable};

/// Bundled OHLC vecs for all periods (time + chain based).
#[derive(Clone, Traversable)]
pub struct OHLCPeriodVecs<T>
where
    T: BytesVecValue + Formattable + Serialize + JsonSchema + 'static,
{
    // Time-based periods
    pub dateindex: EagerVec<BytesVec<DateIndex, T>>,
    pub week: EagerVec<BytesVec<WeekIndex, T>>,
    pub month: EagerVec<BytesVec<MonthIndex, T>>,
    pub quarter: EagerVec<BytesVec<QuarterIndex, T>>,
    pub semester: EagerVec<BytesVec<SemesterIndex, T>>,
    pub year: EagerVec<BytesVec<YearIndex, T>>,
    pub decade: EagerVec<BytesVec<DecadeIndex, T>>,
    // Chain-based periods
    pub height: EagerVec<BytesVec<Height, T>>,
    pub difficultyepoch: EagerVec<BytesVec<DifficultyEpoch, T>>,
}
