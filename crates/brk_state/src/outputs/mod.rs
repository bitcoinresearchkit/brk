mod by_date_range;
mod by_epoch;
mod by_from_date;
mod by_from_size;
mod by_size_range;
mod by_spendable_type;
mod by_term;
mod by_type;
mod by_unspendable_type;
mod by_up_to_date;
mod by_up_to_size;
// mod by_value;
mod filter;

pub use by_date_range::*;
pub use by_epoch::*;
pub use by_from_date::*;
pub use by_from_size::*;
pub use by_size_range::*;
pub use by_spendable_type::*;
pub use by_term::*;
pub use by_type::*;
pub use by_unspendable_type::*;
pub use by_up_to_date::*;
pub use by_up_to_size::*;
// pub use by_value::*;
pub use filter::*;

#[derive(Default, Clone)]
pub struct Outputs<T> {
    pub all: T,
    pub by_date_range: OutputsByDateRange<T>,
    pub by_epoch: OutputsByEpoch<T>,
    pub by_from_date: OutputsByFromDate<T>,
    pub by_from_size: OutputsByFromSize<T>,
    pub by_size_range: OutputsBySizeRange<T>,
    pub by_term: OutputsByTerm<T>,
    pub by_type: OutputsBySpendableType<T>,
    pub by_up_to_date: OutputsByUpToDate<T>,
    pub by_up_to_size: OutputsByUpToSize<T>,
    // Needs whole UTXO set, TODO later
    // pub by_value: OutputsByValue<T>,
}

impl<T> Outputs<T> {
    pub fn as_mut_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to_date.as_mut_vec())
            .chain(self.by_from_date.as_mut_vec())
            .chain(self.by_from_size.as_mut_vec())
            .chain(self.by_date_range.as_mut_vec())
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size_range.as_mut_vec())
            .chain(self.by_up_to_size.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            // .chain(self.by_value.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_separate_vecs(&mut self) -> Vec<&mut T> {
        self.by_date_range
            .as_mut_vec()
            .into_iter()
            .chain(self.by_epoch.as_mut_vec())
            .chain(self.by_size_range.as_mut_vec())
            .chain(self.by_type.as_mut_vec())
            // .chain(self.by_value.as_mut_vec())
            .collect::<Vec<_>>()
    }

    pub fn as_mut_overlaping_vecs(&mut self) -> Vec<&mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.by_term.as_mut_vec())
            .chain(self.by_up_to_date.as_mut_vec())
            .chain(self.by_from_date.as_mut_vec())
            .chain(self.by_up_to_size.as_mut_vec())
            .chain(self.by_from_size.as_mut_vec())
            .collect::<Vec<_>>()
    }
}

impl<T> Outputs<(OutputFilter, T)> {
    pub fn vecs(&self) -> Vec<&T> {
        [&self.all.1]
            .into_iter()
            .chain(self.by_term.vecs())
            .chain(self.by_up_to_date.vecs())
            .chain(self.by_from_date.vecs())
            .chain(self.by_date_range.vecs())
            .chain(self.by_epoch.vecs())
            .chain(self.by_size_range.vecs())
            .chain(self.by_type.vecs())
            .chain(self.by_up_to_size.vecs())
            .chain(self.by_from_size.vecs())
            // .chain(self.by_value.vecs())
            .collect::<Vec<_>>()
    }
}

impl<T> From<Outputs<T>> for Outputs<(OutputFilter, T)> {
    fn from(value: Outputs<T>) -> Self {
        Self {
            all: (OutputFilter::All, value.all),
            by_term: OutputsByTerm::from(value.by_term),
            by_up_to_date: OutputsByUpToDate::from(value.by_up_to_date),
            by_from_date: OutputsByFromDate::from(value.by_from_date),
            by_date_range: OutputsByDateRange::from(value.by_date_range),
            by_epoch: OutputsByEpoch::from(value.by_epoch),
            by_size_range: OutputsBySizeRange::from(value.by_size_range),
            by_up_to_size: OutputsByUpToSize::from(value.by_up_to_size),
            by_from_size: OutputsByFromSize::from(value.by_from_size),
            // Needs whole UTXO set, TODO later
            // by_value: OutputsByValue<T>,
            by_type: OutputsBySpendableType::from(value.by_type),
        }
    }
}
