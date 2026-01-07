use brk_traversable::Traversable;
use brk_types::{
    Close, DateIndex, DecadeIndex, DifficultyEpoch, Height, High, Low, MonthIndex, OHLCSats, Open,
    QuarterIndex, Sats, SemesterIndex, WeekIndex, YearIndex,
};
use vecdb::{BytesVec, EagerVec};

use crate::internal::{
    ComputedChainFirst, ComputedChainLast, ComputedChainMax, ComputedChainMin, ComputedDateLast,
    ComputedVecsDateFirst, ComputedVecsDateMax, ComputedVecsDateMin,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // OHLC in sats
    pub dateindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DateIndex, OHLCSats>>,
    pub height_to_price_ohlc_in_sats: EagerVec<BytesVec<Height, OHLCSats>>,

    // Computed time indexes in sats
    pub timeindexes_to_price_open_in_sats: ComputedVecsDateFirst<Open<Sats>>,
    pub timeindexes_to_price_high_in_sats: ComputedVecsDateMax<High<Sats>>,
    pub timeindexes_to_price_low_in_sats: ComputedVecsDateMin<Low<Sats>>,
    pub timeindexes_to_price_close_in_sats: ComputedDateLast<Close<Sats>>,

    // Computed chain indexes in sats (KISS types)
    pub chainindexes_to_price_open_in_sats: ComputedChainFirst<Open<Sats>>,
    pub chainindexes_to_price_high_in_sats: ComputedChainMax<High<Sats>>,
    pub chainindexes_to_price_low_in_sats: ComputedChainMin<Low<Sats>>,
    pub chainindexes_to_price_close_in_sats: ComputedChainLast<Close<Sats>>,

    // Period OHLC in sats
    pub weekindex_to_price_ohlc_in_sats: EagerVec<BytesVec<WeekIndex, OHLCSats>>,
    pub difficultyepoch_to_price_ohlc_in_sats: EagerVec<BytesVec<DifficultyEpoch, OHLCSats>>,
    pub monthindex_to_price_ohlc_in_sats: EagerVec<BytesVec<MonthIndex, OHLCSats>>,
    pub quarterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<QuarterIndex, OHLCSats>>,
    pub semesterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<SemesterIndex, OHLCSats>>,
    pub yearindex_to_price_ohlc_in_sats: EagerVec<BytesVec<YearIndex, OHLCSats>>,
    pub decadeindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DecadeIndex, OHLCSats>>,
}
