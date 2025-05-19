#![allow(unused)]

use brk_core::OutputType;

use crate::vecs::utxos::Vecs_;

mod by_epoch;
mod by_from;
mod by_range;
mod by_size;
mod by_term;
mod by_type;
mod by_up_to;
mod by_value;

pub use by_epoch::*;
pub use by_from::*;
pub use by_range::*;
pub use by_size::*;
pub use by_term::*;
pub use by_type::*;
pub use by_up_to::*;
pub use by_value::*;

#[derive(Default, Clone)]
pub struct Outputs<T> {
    pub all: T,
    pub by_term: OutputsByTerm<T>,
    pub by_up_to: OutputsByUpTo<T>,
    pub by_from: OutputsByFrom<T>,
    pub by_range: OutputsByRange<T>,
    pub by_epoch: OutputsByEpoch<T>,
    pub by_size: OutputsBySize<T>,
    pub by_value: OutputsByValue<T>,
    pub by_type: OutputsByType<T>,
}

impl<T> Outputs<T> {
    pub fn mut_flatten(&mut self) -> Vec<&mut T> {
        [vec![&mut self.all], self.by_term.mut_flatten()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    }
}
