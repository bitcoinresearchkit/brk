use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

use super::GroupFilter;

#[derive(Default, Clone)]
pub struct ByTerm<T> {
    pub short: T,
    pub long: T,
}

impl<T> ByTerm<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        [&mut self.short, &mut self.long].into_iter()
    }
}

impl<T> ByTerm<(GroupFilter, T)> {
    pub fn iter_right(&self) -> impl Iterator<Item = &T> {
        [&self.short.1, &self.long.1].into_iter()
    }
}

impl<T> From<ByTerm<T>> for ByTerm<(GroupFilter, T)> {
    fn from(value: ByTerm<T>) -> Self {
        Self {
            short: (GroupFilter::LowerThan(5 * 30), value.short),
            long: (GroupFilter::GreaterOrEqual(5 * 30), value.long),
        }
    }
}

impl<T: IVecs> IVecs for ByTerm<(GroupFilter, T)> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [("short", &self.short), ("long", &self.long)]
                .into_iter()
                .map(|(name, (_, field))| (name.to_string(), field.to_tree_node()))
                .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.short.1.iter());
        iter = Box::new(iter.chain(self.long.1.iter()));
        iter
    }
}
