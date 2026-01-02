use brk_traversable::Traversable;
use brk_types::{
    Cents, Close, DateIndex, DecadeIndex, DifficultyEpoch, Dollars, Height, High, Low, MonthIndex,
    OHLCCents, OHLCDollars, Open, QuarterIndex, SemesterIndex, WeekIndex, YearIndex,
};
use vecdb::{BytesVec, EagerVec, LazyVecFrom1};

use crate::internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // Derived price data in cents
    pub dateindex_to_price_close_in_cents: LazyVecFrom1<DateIndex, Close<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_high_in_cents: LazyVecFrom1<DateIndex, High<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_low_in_cents: LazyVecFrom1<DateIndex, Low<Cents>, DateIndex, OHLCCents>,
    pub dateindex_to_price_open_in_cents: LazyVecFrom1<DateIndex, Open<Cents>, DateIndex, OHLCCents>,
    pub height_to_price_close_in_cents: LazyVecFrom1<Height, Close<Cents>, Height, OHLCCents>,
    pub height_to_price_high_in_cents: LazyVecFrom1<Height, High<Cents>, Height, OHLCCents>,
    pub height_to_price_low_in_cents: LazyVecFrom1<Height, Low<Cents>, Height, OHLCCents>,
    pub height_to_price_open_in_cents: LazyVecFrom1<Height, Open<Cents>, Height, OHLCCents>,

    // OHLC in dollars
    pub dateindex_to_price_ohlc: LazyVecFrom1<DateIndex, OHLCDollars, DateIndex, OHLCCents>,
    pub height_to_price_ohlc: LazyVecFrom1<Height, OHLCDollars, Height, OHLCCents>,

    // Computed time indexes
    pub timeindexes_to_price_close: ComputedVecsFromDateIndex<Close<Dollars>>,
    pub timeindexes_to_price_high: ComputedVecsFromDateIndex<High<Dollars>>,
    pub timeindexes_to_price_low: ComputedVecsFromDateIndex<Low<Dollars>>,
    pub timeindexes_to_price_open: ComputedVecsFromDateIndex<Open<Dollars>>,

    // Computed chain indexes
    pub chainindexes_to_price_close: ComputedVecsFromHeightStrict<Close<Dollars>>,
    pub chainindexes_to_price_high: ComputedVecsFromHeightStrict<High<Dollars>>,
    pub chainindexes_to_price_low: ComputedVecsFromHeightStrict<Low<Dollars>>,
    pub chainindexes_to_price_open: ComputedVecsFromHeightStrict<Open<Dollars>>,

    // Period OHLC
    pub weekindex_to_price_ohlc: EagerVec<BytesVec<WeekIndex, OHLCDollars>>,
    pub difficultyepoch_to_price_ohlc: EagerVec<BytesVec<DifficultyEpoch, OHLCDollars>>,
    pub monthindex_to_price_ohlc: EagerVec<BytesVec<MonthIndex, OHLCDollars>>,
    pub quarterindex_to_price_ohlc: EagerVec<BytesVec<QuarterIndex, OHLCDollars>>,
    pub semesterindex_to_price_ohlc: EagerVec<BytesVec<SemesterIndex, OHLCDollars>>,
    pub yearindex_to_price_ohlc: EagerVec<BytesVec<YearIndex, OHLCDollars>>,
    pub decadeindex_to_price_ohlc: EagerVec<BytesVec<DecadeIndex, OHLCDollars>>,
}
