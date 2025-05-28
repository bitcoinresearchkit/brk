mod by_epoch;
mod by_from;
mod by_range;
mod by_size;
mod by_spendable_type;
mod by_term;
mod by_type;
mod by_unspendable_type;
mod by_up_to;
// mod by_value;
mod filter;

pub use by_epoch::*;
pub use by_from::*;
pub use by_range::*;
pub use by_size::*;
pub use by_spendable_type::*;
pub use by_term::*;
pub use by_type::*;
pub use by_unspendable_type::*;
pub use by_up_to::*;
// pub use by_value::*;
pub use filter::*;

#[derive(Default, Clone)]
pub struct Outputs<T> {
    pub all: T,
    pub by_term: OutputsByTerm<T>,
    pub by_up_to: OutputsByUpTo<T>,
    pub by_from: OutputsByFrom<T>,
    pub by_range: OutputsByRange<T>,
    pub by_epoch: OutputsByEpoch<T>,
    pub by_type: OutputsBySpendableType<T>,
    pub by_size: OutputsBySize<T>,
    // // Needs whole UTXO set, TODO later
    // // pub by_value: OutputsByValue<T>,
}

impl<T> Outputs<T> {
    pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to.as_mut_vec())
            .chain(self.by_from.as_mut_vec())
            .chain(self.by_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            // // .chain(self.by_value.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> Outputs<(OutputFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        [&self.all.1]
            .into_iter()
            .chain(self.by_term.vecs())
            .chain(self.by_up_to.vecs())
            .chain(self.by_from.vecs())
            .chain(self.by_range.vecs())
            .chain(self.by_epoch.vecs())
            .chain(self.by_size.vecs())
            // // .chain(self.by_value.vecs())
            .chain(self.by_type.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<Outputs<T>> for Outputs<(OutputFilter, T)> {
    fn from(value: Outputs<T>) -> Self {
        Self {
            all: (OutputFilter::All, value.all),
            by_term: OutputsByTerm::from(value.by_term),
            by_up_to: OutputsByUpTo::from(value.by_up_to),
            by_from: OutputsByFrom::from(value.by_from),
            by_range: OutputsByRange::from(value.by_range),
            by_epoch: OutputsByEpoch::from(value.by_epoch),
            by_size: OutputsBySize::from(value.by_size),
            // // Needs whole UTXO set, TODO later
            // // by_value: OutputsByValue<T>,
            by_type: OutputsBySpendableType::from(value.by_type),
        }
    }
}
