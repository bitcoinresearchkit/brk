use brk_traversable::Traversable;
use brk_types::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCCents, OHLCDollars, Open, QuarterIndex, SemesterIndex, WeekIndex, YearIndex,
};
use vecdb::{BytesVec, EagerVec, LazyVecFrom1};

use crate::internal::{
    ComputedChainFirst, ComputedChainLast, ComputedChainMax, ComputedChainMin, ComputedDateLast,
    ComputedVecsDateFirst, ComputedVecsDateMax, ComputedVecsDateMin,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // Derived price data in cents
    pub dateindex_to_price_close_in_cents:
        LazyVecFrom1<DateIndex, Close<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_high_in_cents:
        LazyVecFrom1<DateIndex, High<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_low_in_cents: LazyVecFrom1<DateIndex, Low<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_open_in_cents:
        LazyVecFrom1<DateIndex, Open<Cents>, DateIndex, OHLCCents>,
    pub height_to_price_close_in_cents: LazyVecFrom1<Height, Close<Cents>, Height, OHLCCents>,
    pub height_to_price_high_in_cents: LazyVecFrom1<Height, High<Cents>, Height, OHLCCents>,
    pub height_to_price_low_in_cents: LazyVecFrom1<Height, Low<Cents>, Height, OHLCCents>,
    pub height_to_price_open_in_cents: LazyVecFrom1<Height, Open<Cents>, Height, OHLCCents>,

    // OHLC in dollars
    pub dateindex_to_price_ohlc: LazyVecFrom1<DateIndex, OHLCDollars, DateIndex, OHLCCents>,
    pub height_to_price_ohlc: LazyVecFrom1<Height, OHLCDollars, Height, OHLCCents>,

    // Computed time indexes
    pub timeindexes_to_price_close: ComputedDateLast<Close<Dollars>>,
    pub timeindexes_to_price_high: ComputedVecsDateMax<High<Dollars>>,
    pub timeindexes_to_price_low: ComputedVecsDateMin<Low<Dollars>>,
    pub timeindexes_to_price_open: ComputedVecsDateFirst<Open<Dollars>>,

    // Computed chain indexes (KISS types)
    pub chainindexes_to_price_close: ComputedChainLast<Close<Dollars>>,
    pub chainindexes_to_price_high: ComputedChainMax<High<Dollars>>,
    pub chainindexes_to_price_low: ComputedChainMin<Low<Dollars>>,
    pub chainindexes_to_price_open: ComputedChainFirst<Open<Dollars>>,

    // Period OHLC
    pub weekindex_to_price_ohlc: EagerVec<BytesVec<WeekIndex, OHLCDollars>>,
    pub difficultyepoch_to_price_ohlc: EagerVec<BytesVec<DifficultyEpoch, OHLCDollars>>,
    pub monthindex_to_price_ohlc: EagerVec<BytesVec<MonthIndex, OHLCDollars>>,
    pub quarterindex_to_price_ohlc: EagerVec<BytesVec<QuarterIndex, OHLCDollars>>,
    pub semesterindex_to_price_ohlc: EagerVec<BytesVec<SemesterIndex, OHLCDollars>>,
    pub yearindex_to_price_ohlc: EagerVec<BytesVec<YearIndex, OHLCDollars>>,
    pub decadeindex_to_price_ohlc: EagerVec<BytesVec<DecadeIndex, OHLCDollars>>,
}
