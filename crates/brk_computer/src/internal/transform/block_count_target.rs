use brk_types::{
    Day1, Day3, DifficultyEpoch, HalvingEpoch, Height, Hour1, Hour12, Hour4, Minute1, Minute10,
    Minute30, Minute5, Month1, Month3, Month6, StoredU64, Week1, Year1, Year10,
};
use vecdb::UnaryTransform;

use crate::blocks::{
    TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DAY3, TARGET_BLOCKS_PER_DECADE,
    TARGET_BLOCKS_PER_HALVING, TARGET_BLOCKS_PER_HOUR1, TARGET_BLOCKS_PER_HOUR12,
    TARGET_BLOCKS_PER_HOUR4, TARGET_BLOCKS_PER_MINUTE1, TARGET_BLOCKS_PER_MINUTE10,
    TARGET_BLOCKS_PER_MINUTE30, TARGET_BLOCKS_PER_MINUTE5, TARGET_BLOCKS_PER_MONTH,
    TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER, TARGET_BLOCKS_PER_WEEK,
    TARGET_BLOCKS_PER_YEAR,
};

pub struct BlockCountTarget;

impl UnaryTransform<Height, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Height) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_DAY)
    }
}

impl UnaryTransform<Minute1, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Minute1) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_MINUTE1)
    }
}

impl UnaryTransform<Minute5, StoredU64> for BlockCountTarget {
    #[inline(always)]
    fn apply(_: Minute5) -> StoredU64 {
        StoredU64::from(TARGET_BLOCKS_PER_MINUTE5)
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
