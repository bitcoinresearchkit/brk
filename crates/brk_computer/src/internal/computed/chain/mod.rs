//! Chain-level computed types (height + difficultyepoch only).
//!
//! These are simpler than block-level types which include dateindex + periods.

mod first;
mod last;
mod max;
mod min;

pub use first::*;
pub use last::*;
pub use max::*;
pub use min::*;
