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
            GroupFilter::All => true,
            GroupFilter::LowerThan(max) => *max > value,
            GroupFilter::GreaterOrEqual(min) => *min <= value,
            GroupFilter::Range(r) => r.contains(&value),
            GroupFilter::Epoch(_) => false,
            GroupFilter::Type(_) => false,
        }
    }

    pub fn includes(&self, other: &GroupFilter) -> bool {
        match self {
            GroupFilter::All => true,
            GroupFilter::LowerThan(max) => match other {
                GroupFilter::All => false,
                GroupFilter::LowerThan(max2) => max >= max2,
                GroupFilter::Range(range) => range.end <= *max,
                GroupFilter::GreaterOrEqual(_) => false,
                GroupFilter::Epoch(_) => false,
                GroupFilter::Type(_) => false,
            },
            GroupFilter::GreaterOrEqual(min) => match other {
                GroupFilter::All => false,
                GroupFilter::LowerThan(_) => false,
                GroupFilter::Range(range) => range.start >= *min,
                GroupFilter::GreaterOrEqual(min2) => min <= min2,
                GroupFilter::Epoch(_) => false,
                GroupFilter::Type(_) => false,
            },
            GroupFilter::Range(_) => false,
            GroupFilter::Epoch(_) => false,
            GroupFilter::Type(_) => false,
        }
    }
}
