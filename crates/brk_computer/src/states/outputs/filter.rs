use std::ops::Range;

use brk_core::{HalvingEpoch, OutputType, Sats};

#[derive(Clone)]
pub enum OutputFilter {
    All,
    To(usize),
    Range(Range<usize>),
    From(usize),
    Size(Range<Sats>),
    Epoch(HalvingEpoch),
    Type(OutputType),
}
