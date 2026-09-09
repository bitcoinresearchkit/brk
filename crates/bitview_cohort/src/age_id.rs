use std::ops::Range;

use crate::{
    AgeRangeId, CohortName, OVER_AGE_HOURS, OVER_AGE_NAMES, OverAgeId, UNDER_AGE_HOURS,
    UNDER_AGE_NAMES, UnderAgeId,
};

/// A supported age range or overlapping age threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgeId {
    Range(AgeRangeId),
    Under(UnderAgeId),
    Over(OverAgeId),
}

impl AgeId {
    pub fn bounds(self) -> Range<usize> {
        match self {
            Self::Range(id) => id.bounds().clone(),
            Self::Under(id) => 0..*id.select(&UNDER_AGE_HOURS),
            Self::Over(id) => *id.select(&OVER_AGE_HOURS)..usize::MAX,
        }
    }

    pub fn name(self) -> &'static CohortName {
        match self {
            Self::Range(id) => id.name(),
            Self::Under(id) => id.select(&UNDER_AGE_NAMES),
            Self::Over(id) => id.select(&OVER_AGE_NAMES),
        }
    }
}
