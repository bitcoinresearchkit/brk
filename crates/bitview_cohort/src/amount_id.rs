use std::ops::Range;

use brk_types::Sats;

use crate::{
    AMOUNT_RANGE_BOUNDS, AMOUNT_RANGE_NAMES, AmountRangeId, CohortName, OVER_AMOUNT_NAMES,
    OVER_AMOUNT_THRESHOLDS, OverAmountId, UNDER_AMOUNT_NAMES, UNDER_AMOUNT_THRESHOLDS,
    UnderAmountId,
};

/// A supported amount range or overlapping amount threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmountId {
    Range(AmountRangeId),
    Under(UnderAmountId),
    Over(OverAmountId),
}

impl AmountId {
    pub fn bounds(self) -> Range<Sats> {
        match self {
            Self::Range(id) => id.select(&AMOUNT_RANGE_BOUNDS).clone(),
            Self::Under(id) => Sats::ZERO..*id.select(&UNDER_AMOUNT_THRESHOLDS),
            Self::Over(id) => *id.select(&OVER_AMOUNT_THRESHOLDS)..Sats::MAX,
        }
    }

    pub fn name(self) -> &'static CohortName {
        match self {
            Self::Range(id) => id.select(&AMOUNT_RANGE_NAMES),
            Self::Under(id) => id.select(&UNDER_AMOUNT_NAMES),
            Self::Over(id) => id.select(&OVER_AMOUNT_NAMES),
        }
    }

    /// Disjoint amount ranges making up this cohort, in collection order.
    pub fn ranges(self) -> impl Iterator<Item = AmountRangeId> {
        let bounds = self.bounds();
        AmountRangeId::ALL.iter().copied().filter(move |id| {
            let range = id.select(&AMOUNT_RANGE_BOUNDS);
            range.start >= bounds.start && range.end <= bounds.end
        })
    }
}
