//! Lazy aggregation primitives (finer index → coarser index).

mod average;
mod cumulative;
mod first;
mod full;
mod last;
mod max;
mod min;
mod spread;
mod sum;
mod sum_cum;

pub use average::*;
pub use cumulative::*;
pub use first::*;
pub use full::*;
pub use last::*;
pub use max::*;
pub use min::*;
pub use spread::*;
pub use sum::*;
pub use sum_cum::*;
