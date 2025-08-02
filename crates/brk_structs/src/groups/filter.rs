use std::ops::Range;

use crate::{HalvingEpoch, OutputType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupFilter {
    All,
    LowerThan(usize),
    Range(Range<usize>),
    GreaterOrEqual(usize),
    Epoch(HalvingEpoch),
    Type(OutputType),
}

impl GroupFilter {
    pub fn contains(&self, value: usize) -> bool {
        match self {
            GroupFilter::Range(r) => r.contains(&value),
            GroupFilter::LowerThan(max) => *max > value,
            GroupFilter::GreaterOrEqual(min) => *min <= value,
            GroupFilter::All => true,
            GroupFilter::Epoch(_) | GroupFilter::Type(_) => false,
        }
    }

    pub fn includes(&self, other: &GroupFilter) -> bool {
        match self {
            GroupFilter::All => true,
            GroupFilter::LowerThan(max) => match other {
                GroupFilter::LowerThan(max2) => max >= max2,
                GroupFilter::Range(range) => range.end <= *max,
                GroupFilter::All
                | GroupFilter::GreaterOrEqual(_)
                | GroupFilter::Epoch(_)
                | GroupFilter::Type(_) => false,
            },
            GroupFilter::GreaterOrEqual(min) => match other {
                GroupFilter::Range(range) => range.start >= *min,
                GroupFilter::GreaterOrEqual(min2) => min <= min2,
                GroupFilter::All
                | GroupFilter::LowerThan(_)
                | GroupFilter::Epoch(_)
                | GroupFilter::Type(_) => false,
            },
            GroupFilter::Range(_) | GroupFilter::Epoch(_) | GroupFilter::Type(_) => false,
        }
    }
}
