use std::ops::Range;

use brk_core::{HalvingEpoch, OutputType};

#[derive(Debug, Clone)]
pub enum OutputFilter {
    All,
    To(usize),
    Range(Range<usize>),
    From(usize),
    Epoch(HalvingEpoch),
    Type(OutputType),
}

impl OutputFilter {
    pub fn contains(&self, value: usize) -> bool {
        match self {
            OutputFilter::All => true,
            OutputFilter::To(to) => *to > value,
            OutputFilter::From(from) => *from <= value,
            OutputFilter::Range(r) => r.contains(&value),
            OutputFilter::Epoch(_) => false,
            OutputFilter::Type(_) => false,
        }
    }

    pub fn includes(&self, other: &OutputFilter) -> bool {
        match self {
            OutputFilter::All => true,
            OutputFilter::To(to) => match other {
                OutputFilter::All => false,
                OutputFilter::To(to2) => to >= to2,
                OutputFilter::Range(range) => range.end <= *to,
                OutputFilter::From(_) => true,
                OutputFilter::Epoch(_) => false,
                OutputFilter::Type(_) => false,
            },
            OutputFilter::From(from) => match other {
                OutputFilter::All => false,
                OutputFilter::To(_) => false,
                OutputFilter::Range(range) => range.start >= *from,
                OutputFilter::From(from2) => from <= from2,
                OutputFilter::Epoch(_) => false,
                OutputFilter::Type(_) => false,
            },
            OutputFilter::Range(_) => false,
            OutputFilter::Epoch(_) => false,
            OutputFilter::Type(_) => false,
        }
    }
}
