use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

use crate::{
    ByAgeRange, ByAmountRange, ByEpoch, ByGreatEqualAmount, ByLowerThanAmount, ByMaxAge, ByMinAge,
    BySpendableType, ByTerm, GroupFilter,
};

#[derive(Default, Clone)]
pub struct UTXOGroups<T> {
    pub all: T,
    pub age_range: ByAgeRange<T>,
    pub epoch: ByEpoch<T>,
    pub min_age: ByMinAge<T>,
    pub ge_amount: ByGreatEqualAmount<T>,
    pub amount_range: ByAmountRange<T>,
    pub term: ByTerm<T>,
    pub _type: BySpendableType<T>,
    pub max_age: ByMaxAge<T>,
    pub lt_amount: ByLowerThanAmount<T>,
}

impl<T> UTXOGroups<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.max_age.iter_mut())
            .chain(self.min_age.iter_mut())
            .chain(self.ge_amount.iter_mut())
            .chain(self.age_range.iter_mut())
            .chain(self.epoch.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self.lt_amount.iter_mut())
            .chain(self._type.iter_mut())
    }

    pub fn iter_separate_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.age_range
            .iter_mut()
            .chain(self.epoch.iter_mut())
            .chain(self.amount_range.iter_mut())
            .chain(self._type.iter_mut())
    }

    pub fn iter_overlapping_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.all]
            .into_iter()
            .chain(self.term.iter_mut())
            .chain(self.max_age.iter_mut())
            .chain(self.min_age.iter_mut())
            .chain(self.lt_amount.iter_mut())
            .chain(self.ge_amount.iter_mut())
    }
}

impl<T> UTXOGroups<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self.all.1]
            .into_iter()
            .chain(self.term.iter_right())
            .chain(self.max_age.iter_right())
            .chain(self.min_age.iter_right())
            .chain(self.age_range.iter_right())
            .chain(self.epoch.iter_right())
            .chain(self.amount_range.iter_right())
            .chain(self._type.iter_right())
            .chain(self.lt_amount.iter_right())
            .chain(self.ge_amount.iter_right())
    }
}

impl<T> From<UTXOGroups<T>> for UTXOGroups<(GroupFilter, T)> {
    fn from(value: UTXOGroups<T>) -> Self {
        Self {
            all: (GroupFilter::All, value.all),
            term: ByTerm::from(value.term),
            max_age: ByMaxAge::from(value.max_age),
            min_age: ByMinAge::from(value.min_age),
            age_range: ByAgeRange::from(value.age_range),
            epoch: ByEpoch::from(value.epoch),
            amount_range: ByAmountRange::from(value.amount_range),
            lt_amount: ByLowerThanAmount::from(value.lt_amount),
            ge_amount: ByGreatEqualAmount::from(value.ge_amount),
            _type: BySpendableType::from(value._type),
        }
    }
}

impl<T: IVecs> IVecs for UTXOGroups<(GroupFilter, T)> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                ("all", self.all.1.to_tree_node()),
                ("age_range", self.age_range.to_tree_node()),
                ("epoch", self.epoch.to_tree_node()),
                ("min_age", self.min_age.to_tree_node()),
                ("ge_amount", self.ge_amount.to_tree_node()),
                ("amount_range", self.amount_range.to_tree_node()),
                ("term", self.term.to_tree_node()),
                ("type", self._type.to_tree_node()),
                ("max_age", self.max_age.to_tree_node()),
                ("lt_amount", self.lt_amount.to_tree_node()),
            ]
            .into_iter()
            .map(|(name, node)| (name.to_string(), node))
            .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.all.1.iter());
        iter = Box::new(iter.chain(IVecs::iter(&self.age_range)));
        iter = Box::new(iter.chain(self.epoch.iter()));
        iter = Box::new(iter.chain(self.min_age.iter()));
        iter = Box::new(iter.chain(IVecs::iter(&self.ge_amount)));
        iter = Box::new(iter.chain(IVecs::iter(&self.amount_range)));
        iter = Box::new(iter.chain(self.term.iter()));
        iter = Box::new(iter.chain(self._type.iter()));
        iter = Box::new(iter.chain(self.max_age.iter()));
        iter = Box::new(iter.chain(self.lt_amount.iter()));
        iter
    }
}
