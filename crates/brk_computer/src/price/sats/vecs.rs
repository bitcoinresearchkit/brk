use brk_traversable::Traversable;
use brk_types::{
    Close, DateIndex, DecadeIndex, DifficultyEpoch, Height, High, Low, MonthIndex,
    OHLCSats, Open, QuarterIndex, Sats, SemesterIndex, WeekIndex, YearIndex,
};
use vecdb::{BytesVec, EagerVec};

use crate::internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeightStrict};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // OHLC in sats
    pub dateindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DateIndex, OHLCSats>>,
    pub height_to_price_ohlc_in_sats: EagerVec<BytesVec<Height, OHLCSats>>,

    // Computed time indexes in sats
    pub timeindexes_to_price_open_in_sats: ComputedVecsFromDateIndex<Open<Sats>>,
    pub timeindexes_to_price_high_in_sats: ComputedVecsFromDateIndex<High<Sats>>,
    pub timeindexes_to_price_low_in_sats: ComputedVecsFromDateIndex<Low<Sats>>,
    pub timeindexes_to_price_close_in_sats: ComputedVecsFromDateIndex<Close<Sats>>,

    // Computed chain indexes in sats
    pub chainindexes_to_price_open_in_sats: ComputedVecsFromHeightStrict<Open<Sats>>,
    pub chainindexes_to_price_high_in_sats: ComputedVecsFromHeightStrict<High<Sats>>,
    pub chainindexes_to_price_low_in_sats: ComputedVecsFromHeightStrict<Low<Sats>>,
    pub chainindexes_to_price_close_in_sats: ComputedVecsFromHeightStrict<Close<Sats>>,

    // Period OHLC in sats
    pub weekindex_to_price_ohlc_in_sats: EagerVec<BytesVec<WeekIndex, OHLCSats>>,
    pub difficultyepoch_to_price_ohlc_in_sats: EagerVec<BytesVec<DifficultyEpoch, OHLCSats>>,
    pub monthindex_to_price_ohlc_in_sats: EagerVec<BytesVec<MonthIndex, OHLCSats>>,
    pub quarterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<QuarterIndex, OHLCSats>>,
    pub semesterindex_to_price_ohlc_in_sats: EagerVec<BytesVec<SemesterIndex, OHLCSats>>,
    pub yearindex_to_price_ohlc_in_sats: EagerVec<BytesVec<YearIndex, OHLCSats>>,
    pub decadeindex_to_price_ohlc_in_sats: EagerVec<BytesVec<DecadeIndex, OHLCSats>>,
}
