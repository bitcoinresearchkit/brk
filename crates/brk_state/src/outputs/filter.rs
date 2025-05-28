use std::ops::Range;

use brk_core::{HalvingEpoch, OutputType};

#[derive(Debug, Clone)]
pub enum OutputFilter {
    All,
    To(usize),
    Range(Range<usize>),
    From(usize),
    Size,
    Epoch(HalvingEpoch),
    Type(OutputType),
}
