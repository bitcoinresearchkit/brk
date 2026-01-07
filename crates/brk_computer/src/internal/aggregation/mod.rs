//! Lazy aggregation primitives (finer index → coarser index).

mod average;
mod cumulative;
mod distribution;
mod first;
mod last;
mod max;
mod min;
mod stats_aggregate;
mod sum;
mod sum_cum;

pub use average::*;
pub use cumulative::*;
pub use distribution::*;
pub use first::*;
pub use last::*;
pub use max::*;
pub use min::*;
pub use stats_aggregate::*;
pub use sum::*;
pub use sum_cum::*;
