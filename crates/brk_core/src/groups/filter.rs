use std::ops::Range;

use crate::{HalvingEpoch, OutputType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupFilter {
    All,
    To(usize),
    Range(Range<usize>),
    From(usize),
    Epoch(HalvingEpoch),
    Type(OutputType),
}

impl GroupFilter {
    pub fn contains(&self, value: usize) -> bool {
        match self {
            GroupFilter::All => true,
            GroupFilter::To(to) => *to > value,
            GroupFilter::From(from) => *from <= value,
            GroupFilter::Range(r) => r.contains(&value),
            GroupFilter::Epoch(_) => false,
            GroupFilter::Type(_) => false,
        }
    }

    pub fn includes(&self, other: &GroupFilter) -> bool {
        match self {
            GroupFilter::All => true,
            GroupFilter::To(to) => match other {
                GroupFilter::All => false,
                GroupFilter::To(to2) => to >= to2,
                GroupFilter::Range(range) => range.end <= *to,
                GroupFilter::From(_) => false,
                GroupFilter::Epoch(_) => false,
                GroupFilter::Type(_) => false,
            },
            GroupFilter::From(from) => match other {
                GroupFilter::All => false,
                GroupFilter::To(_) => false,
                GroupFilter::Range(range) => range.start >= *from,
                GroupFilter::From(from2) => from <= from2,
                GroupFilter::Epoch(_) => false,
                GroupFilter::Type(_) => false,
            },
            GroupFilter::Range(_) => false,
            GroupFilter::Epoch(_) => false,
            GroupFilter::Type(_) => false,
        }
    }
}
