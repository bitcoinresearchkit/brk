#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    AgeId, CohortId, CohortName, HOURS_1D, HOURS_1M, HOURS_1W, HOURS_1Y, HOURS_2M, HOURS_2Y,
    HOURS_3M, HOURS_3Y, HOURS_4M, HOURS_4Y, HOURS_5M, HOURS_5Y, HOURS_6M, HOURS_6Y, HOURS_7Y,
    HOURS_8Y, HOURS_9M, HOURS_10Y, HOURS_12Y, HOURS_18M,
};

/// Over-age thresholds in hours
pub const OVER_AGE_HOURS: OverAge<usize> = OverAge {
    _1d: HOURS_1D,
    _1w: HOURS_1W,
    _1m: HOURS_1M,
    _2m: HOURS_2M,
    _3m: HOURS_3M,
    _4m: HOURS_4M,
    _5m: HOURS_5M,
    _6m: HOURS_6M,
    _9m: HOURS_9M,
    _1y: HOURS_1Y,
    _18m: HOURS_18M,
    _2y: HOURS_2Y,
    _3y: HOURS_3Y,
    _4y: HOURS_4Y,
    _5y: HOURS_5Y,
    _6y: HOURS_6Y,
    _7y: HOURS_7Y,
    _8y: HOURS_8Y,
    _10y: HOURS_10Y,
    _12y: HOURS_12Y,
};

/// Over-age names
pub const OVER_AGE_NAMES: OverAge<CohortName> = OverAge {
    _1d: CohortName::new("over_1d_old", "1d+", "Over 1 Day Old"),
    _1w: CohortName::new("over_1w_old", "1w+", "Over 1 Week Old"),
    _1m: CohortName::new("over_1m_old", "1m+", "Over 1 Month Old"),
    _2m: CohortName::new("over_2m_old", "2m+", "Over 2 Months Old"),
    _3m: CohortName::new("over_3m_old", "3m+", "Over 3 Months Old"),
    _4m: CohortName::new("over_4m_old", "4m+", "Over 4 Months Old"),
    _5m: CohortName::new("over_5m_old", "5m+", "Over 5 Months Old"),
    _6m: CohortName::new("over_6m_old", "6m+", "Over 6 Months Old"),
    _9m: CohortName::new("over_9m_old", "9m+", "Over 9 Months Old"),
    _1y: CohortName::new("over_1y_old", "1y+", "Over 1 Year Old"),
    _18m: CohortName::new("over_18m_old", "18m+", "Over 18 Months Old"),
    _2y: CohortName::new("over_2y_old", "2y+", "Over 2 Years Old"),
    _3y: CohortName::new("over_3y_old", "3y+", "Over 3 Years Old"),
    _4y: CohortName::new("over_4y_old", "4y+", "Over 4 Years Old"),
    _5y: CohortName::new("over_5y_old", "5y+", "Over 5 Years Old"),
    _6y: CohortName::new("over_6y_old", "6y+", "Over 6 Years Old"),
    _7y: CohortName::new("over_7y_old", "7y+", "Over 7 Years Old"),
    _8y: CohortName::new("over_8y_old", "8y+", "Over 8 Years Old"),
    _10y: CohortName::new("over_10y_old", "10y+", "Over 10 Years Old"),
    _12y: CohortName::new("over_12y_old", "12y+", "Over 12 Years Old"),
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct OverAge<T> {
    /// Uses UTXOs at least 1 day old.
    pub _1d: T,
    /// Uses UTXOs at least 7 days old.
    pub _1w: T,
    /// Uses UTXOs at least 30 days old.
    pub _1m: T,
    /// Uses UTXOs at least 60 days old.
    pub _2m: T,
    /// Uses UTXOs at least 90 days old.
    pub _3m: T,
    /// Uses UTXOs at least 120 days old.
    pub _4m: T,
    /// Uses UTXOs at least 150 days old.
    pub _5m: T,
    /// Uses UTXOs at least 180 days old.
    pub _6m: T,
    /// Uses UTXOs at least 270 days old.
    pub _9m: T,
    /// Uses UTXOs at least 365 days old.
    pub _1y: T,
    /// Uses UTXOs at least 545 days old.
    pub _18m: T,
    /// Uses UTXOs at least 2 years old, using 365-day years.
    pub _2y: T,
    /// Uses UTXOs at least 3 years old, using 365-day years.
    pub _3y: T,
    /// Uses UTXOs at least 4 years old, using 365-day years.
    pub _4y: T,
    /// Uses UTXOs at least 5 years old, using 365-day years.
    pub _5y: T,
    /// Uses UTXOs at least 6 years old, using 365-day years.
    pub _6y: T,
    /// Uses UTXOs at least 7 years old, using 365-day years.
    pub _7y: T,
    /// Uses UTXOs at least 8 years old, using 365-day years.
    pub _8y: T,
    /// Uses UTXOs at least 10 years old, using 365-day years.
    pub _10y: T,
    /// Uses UTXOs at least 12 years old, using 365-day years.
    pub _12y: T,
}

define_cohort_id!(
    OverAgeId for OverAge {
        Over1D => _1d,
        Over1W => _1w,
        Over1M => _1m,
        Over2M => _2m,
        Over3M => _3m,
        Over4M => _4m,
        Over5M => _5m,
        Over6M => _6m,
        Over9M => _9m,
        Over1Y => _1y,
        Over18M => _18m,
        Over2Y => _2y,
        Over3Y => _3y,
        Over4Y => _4y,
        Over5Y => _5y,
        Over6Y => _6y,
        Over7Y => _7y,
        Over8Y => _8y,
        Over10Y => _10y,
        Over12Y => _12y,
    }
);

impl OverAge<CohortName> {
    pub const fn names() -> &'static Self {
        &OVER_AGE_NAMES
    }
}

impl<T> OverAge<T> {
    pub fn new(mut create: impl FnMut(CohortId) -> T) -> Self {
        Self::from_fn(|id| create(id.cohort()))
    }

    pub fn try_new<E>(mut create: impl FnMut(CohortId) -> Result<T, E>) -> Result<Self, E> {
        Self::try_from_fn(|id| create(id.cohort()))
    }
}

impl OverAgeId {
    pub const fn cohort(self) -> CohortId {
        CohortId::Age(AgeId::Over(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgeRangeId;

    #[test]
    fn new_thresholds_include_only_older_ranges() {
        let nine_months: Vec<_> = OverAgeId::Over9M.cohort().age_ranges().unwrap().collect();
        assert!(!nine_months.contains(&AgeRangeId::From6MTo9M));
        assert!(nine_months.contains(&AgeRangeId::From9MTo1Y));
        let eighteen_months: Vec<_> = OverAgeId::Over18M.cohort().age_ranges().unwrap().collect();
        assert!(!eighteen_months.contains(&AgeRangeId::From1YTo18M));
        assert!(eighteen_months.contains(&AgeRangeId::From18MTo2Y));
    }
}
