//! Lazy aggregation primitives (finer index → coarser index).

mod average;
mod cumulative;
mod distribution;
mod first;
mod last;
mod max;
mod min;
mod full;
mod sum;
mod sum_cum;

pub use average::*;
pub use cumulative::*;
pub use distribution::*;
pub use first::*;
pub use last::*;
pub use max::*;
pub use min::*;
pub use full::*;
pub use sum::*;
pub use sum_cum::*;
