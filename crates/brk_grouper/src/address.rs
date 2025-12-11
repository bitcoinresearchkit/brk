use brk_traversable::Traversable;
use rayon::prelude::*;
use vecdb::AnyExportableVec;

use crate::Filter;

use super::{ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount};

#[derive(Default, Clone)]
pub struct AddressGroups<T> {
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn new<F>(mut create: F) -> Self
    where
        F: FnMut(Filter) -> T,
    {
        Self {
            ge_amount: ByGreatEqualAmount::new(&mut create),
            amount_range: ByAmountRange::new(&mut create),
            lt_amount: ByLowerThanAmount::new(&mut create),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.ge_amount
            .iter()
            .chain(self.amount_range.iter())
            .chain(self.lt_amount.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.ge_amount
            .iter_mut()
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.ge_amount
            .par_iter_mut()
            .chain(self.amount_range.par_iter_mut())
            .chain(self.lt_amount.par_iter_mut())
    }

    pub fn iter_separate(&self) -> impl Iterator<Item = &T> {
        self.amount_range.iter()
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.amount_range.iter_mut()
    }

    pub fn par_iter_separate_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.amount_range.par_iter_mut()
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.lt_amount.iter_mut().chain(self.ge_amount.iter_mut())
    }
}

impl<T> Traversable for AddressGroups<T>
where
    ByGreatEqualAmount<T>: brk_traversable::Traversable,
    ByAmountRange<T>: brk_traversable::Traversable,
    ByLowerThanAmount<T>: brk_traversable::Traversable,
    T: Send + Sync,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::Branch(
            [
                (String::from("ge_amount"), self.ge_amount.to_tree_node()),
                (
                    String::from("amount_range"),
                    self.amount_range.to_tree_node(),
                ),
                (String::from("lt_amount"), self.lt_amount.to_tree_node()),
            ]
            .into(),
        )
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        [
            Box::new(self.ge_amount.iter_any_exportable())
                as Box<dyn Iterator<Item = &dyn AnyExportableVec>>,
            Box::new(self.amount_range.iter_any_exportable())
                as Box<dyn Iterator<Item = &dyn AnyExportableVec>>,
            Box::new(self.lt_amount.iter_any_exportable())
                as Box<dyn Iterator<Item = &dyn AnyExportableVec>>,
        ]
        .into_iter()
        .flatten()
    }
}
