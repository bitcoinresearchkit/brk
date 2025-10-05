use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

use super::{ByAmountRange, ByGreatEqualAmount, ByLowerThanAmount, GroupFilter};

#[derive(Default, Clone)]
pub struct AddressGroups<T> {
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> AddressGroups<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.ge_amount
            .iter_mut()
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.amount_range.iter_mut()
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.lt_amount.iter_mut().chain(self.ge_amount.iter_mut())
    }
}

impl<T> AddressGroups<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        self.amount_range
            .iter_right()
            .chain(self.lt_amount.iter_right())
            .chain(self.ge_amount.iter_right())
    }
}

impl<T> From<AddressGroups<T>> for AddressGroups<(GroupFilter, T)> {
    fn from(value: AddressGroups<T>) -> Self {
        Self {
            amount_range: ByAmountRange::from(value.amount_range),
            lt_amount: ByLowerThanAmount::from(value.lt_amount),
            ge_amount: ByGreatEqualAmount::from(value.ge_amount),
        }
    }
}

impl<T: IVecs> IVecs for AddressGroups<(GroupFilter, T)> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                ("ge_amount", self.ge_amount.to_tree_node()),
                ("amount_range", self.amount_range.to_tree_node()),
                ("lt_amount", self.lt_amount.to_tree_node()),
            ]
            .into_iter()
            .map(|(name, node)| (name.to_string(), node))
            .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.ge_amount.iter());
        iter = Box::new(iter.chain(IVecs::iter(&self.amount_range)));
        iter = Box::new(iter.chain(self.lt_amount.iter()));
        iter
    }
}
