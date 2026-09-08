#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use rayon::prelude::*;

use crate::{
    Amount, ByTerm, Filter, SPENDABLE_TYPE_FILTERS, SPENDABLE_TYPE_NAMES, SpendableType,
    TERM_FILTERS, TERM_NAMES, UTXOGroupCore,
};
use derive_more::{Deref, DerefMut};

#[derive(Default, Clone, Deref, DerefMut)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct UTXOGroups<T> {
    #[deref]
    #[deref_mut]
    #[cfg_attr(feature = "storage", traversable(flatten))]
    pub core: UTXOGroupCore<T>,
    /// Groups UTXOs by their individual output value.
    pub utxo_amount: Amount<T>,
    pub term: ByTerm<T>,
    #[cfg_attr(feature = "storage", traversable(rename = "type"))]
    pub type_: SpendableType<T>,
}

impl<T> UTXOGroups<T> {
    pub fn get(&self, filter: &Filter) -> Option<&T> {
        match filter {
            Filter::Term(term) => match term {
                crate::Term::Sth => Some(&self.term.short),
                crate::Term::Lth => Some(&self.term.long),
            },
            Filter::Amount(_) => self.utxo_amount.get(filter),
            Filter::Type(output_type) => Some(self.type_.get(*output_type)),
            _ => self.core.get(filter),
        }
    }

    pub fn map_named<U>(
        &self,
        mut map: impl FnMut(&Filter, &'static str, &T) -> U,
    ) -> UTXOGroups<U> {
        UTXOGroups {
            core: self.core.map_named(&mut map),
            utxo_amount: self.utxo_amount.map_named(&mut map),
            term: ByTerm::from_fn(|id| {
                map(
                    id.select(&TERM_FILTERS),
                    id.select(&TERM_NAMES).id,
                    id.select(&self.term),
                )
            }),
            type_: SpendableType::from_fn(|id| {
                map(
                    id.select(&SPENDABLE_TYPE_FILTERS),
                    id.select(&SPENDABLE_TYPE_NAMES).id,
                    id.select(&self.type_),
                )
            }),
        }
    }

    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter, &'static str) -> T,
    {
        Self {
            core: UTXOGroupCore::new(&mut create),
            utxo_amount: Amount::new(&mut create),
            term: ByTerm::new(&mut create),
            type_: SpendableType::new(&mut create),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        [&self.core.all]
            .into_iter()
            .chain(self.term.iter())
            .chain(self.core.age.under.iter())
            .chain(self.core.age.over.iter())
            .chain(self.utxo_amount.iter())
            .chain(self.core.age.range.iter())
            .chain(self.core.epoch.iter())
            .chain(self.core.class.iter())
            .chain(self.core.entry.iter())
            .chain(self.type_.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.core.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.core.age.under.iter_mut())
            .chain(self.core.age.over.iter_mut())
            .chain(self.utxo_amount.iter_mut())
            .chain(self.core.age.range.iter_mut())
            .chain(self.core.epoch.iter_mut())
            .chain(self.core.class.iter_mut())
            .chain(self.core.entry.iter_mut())
            .chain(self.type_.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [&mut self.core.all]
            .into_par_iter()
            .chain(self.term.par_iter_mut())
            .chain(self.core.age.under.par_iter_mut())
            .chain(self.core.age.over.par_iter_mut())
            .chain(self.utxo_amount.par_iter_mut())
            .chain(self.core.age.range.par_iter_mut())
            .chain(self.core.epoch.par_iter_mut())
            .chain(self.core.class.par_iter_mut())
            .chain(self.core.entry.par_iter_mut())
            .chain(self.type_.par_iter_mut())
    }

    pub fn iter_separate(&self) -> impl Iterator<Item = &T> {
        self.core
            .age
            .range
            .iter()
            .chain(self.core.epoch.iter())
            .chain(self.core.class.iter())
            .chain(self.core.entry.iter())
            .chain(self.utxo_amount.range.iter())
            .chain(self.type_.iter())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.core
            .age
            .range
            .iter_mut()
            .chain(self.core.epoch.iter_mut())
            .chain(self.core.class.iter_mut())
            .chain(self.core.entry.iter_mut())
            .chain(self.utxo_amount.range.iter_mut())
            .chain(self.type_.iter_mut())
    }

    pub fn par_iter_separate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.core
            .age
            .range
            .par_iter_mut()
            .chain(self.core.epoch.par_iter_mut())
            .chain(self.core.class.par_iter_mut())
            .chain(self.core.entry.par_iter_mut())
            .chain(self.utxo_amount.range.par_iter_mut())
            .chain(self.type_.par_iter_mut())
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.core.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.core.age.under.iter_mut())
            .chain(self.core.age.over.iter_mut())
            .chain(self.utxo_amount.under.iter_mut())
            .chain(self.utxo_amount.over.iter_mut())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate(&self) -> impl Iterator<Item = &T> {
        [&self.core.all].into_iter().chain(self.term.iter())
    }

    pub fn par_iter_aggregate(&self) -> impl ParallelIterator<Item = &T>
    where
        T: Send + Sync,
    {
        [&self.core.all].into_par_iter().chain(self.term.par_iter())
    }

    /// Iterator over aggregate cohorts (all, sth, lth) that compute values from sub-cohorts.
    /// These are cohorts with StateLevel::PriceOnly that derive values from stateful sub-cohorts.
    pub fn iter_aggregate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.core.all].into_iter().chain(self.term.iter_mut())
    }

    pub fn par_iter_aggregate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        [&mut self.core.all]
            .into_par_iter()
            .chain(self.term.par_iter_mut())
    }
}
