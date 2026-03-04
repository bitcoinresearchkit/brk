use brk_types::{
    Close, Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, High, Hour1, Hour4, Hour12, Low,
    Minute10, Minute30, Month1, Month3, Month6, OHLCCents, OHLCDollars, OHLCSats, Open, StoredU64,
    Week1, Year1, Year10,
};
use vecdb::UnaryTransform;

use super::CentsUnsignedToSats;
use crate::blocks::{
    TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DAY3, TARGET_BLOCKS_PER_DECADE,
    TARGET_BLOCKS_PER_HALVING, TARGET_BLOCKS_PER_HOUR1, TARGET_BLOCKS_PER_HOUR4,
    TARGET_BLOCKS_PER_HOUR12, TARGET_BLOCKS_PER_MINUTE10, TARGET_BLOCKS_PER_MINUTE30,
    TARGET_BLOCKS_PER_MONTH, TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER,
    TARGET_BLOCKS_PER_WEEK, TARGET_BLOCKS_PER_YEAR,
};

pub struct BlockCountTarget;

impl UnaryTransform<Height, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Height) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_DAY)
    }
}

impl UnaryTransform<Minute10, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Minute10) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_MINUTE10)
    }
}

impl UnaryTransform<Minute30, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Minute30) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_MINUTE30)
    }
}

impl UnaryTransform<Hour1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Hour1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_HOUR1)
    }
}

impl UnaryTransform<Hour4, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Hour4) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_HOUR4)
    }
}

impl UnaryTransform<Hour12, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Hour12) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_HOUR12)
    }
}

impl UnaryTransform<Day1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Day1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_DAY)
    }
}

impl UnaryTransform<Day3, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Day3) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_DAY3)
    }
}

impl UnaryTransform<Week1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Week1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_WEEK)
    }
}

impl UnaryTransform<Month1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Month1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_MONTH)
    }
}

impl UnaryTransform<Month3, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Month3) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_QUARTER)
    }
}

impl UnaryTransform<Month6, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Month6) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)
    }
}

impl UnaryTransform<Year1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Year1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_YEAR)
    }
}

impl UnaryTransform<Year10, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Year10) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_DECADE)
    }
}

impl UnaryTransform<HalvingEpoch, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: HalvingEpoch) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_HALVING)
    }
}

impl UnaryTransform<DifficultyEpoch, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: DifficultyEpoch) -> StoredU64 {
        StoredU64::from(2016u64)
    }
}

pub struct OhlcCentsToDollars;

impl UnaryTransform<OHLCCents, OHLCDollars> for OhlcCentsToDollars {
    #[inline(always)]
    fn apply(cents: OHLCCents) -> OHLCDollars {
        OHLCDollars::from(cents)
    }
}

pub struct OhlcCentsToSats;

impl UnaryTransform<OHLCCents, OHLCSats> for OhlcCentsToSats {
    #[inline(always)]
    fn apply(cents: OHLCCents) -> OHLCSats {
        OHLCSats {
            open: Open::new(CentsUnsignedToSats::apply(*cents.open)),
            high: High::new(CentsUnsignedToSats::apply(*cents.low)),
            low: Low::new(CentsUnsignedToSats::apply(*cents.high)),
            close: Close::new(CentsUnsignedToSats::apply(*cents.close)),
        }
    }
}
