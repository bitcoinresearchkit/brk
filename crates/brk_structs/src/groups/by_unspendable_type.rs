use std::ops::{Add, AddAssign};

use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

#[derive(Default, Clone, Debug)]
pub struct ByUnspendableType<T> {
    pub opreturn: T,
}

impl<T> ByUnspendableType<T> {
    pub fn as_vec(&self) -> [&T; 1] {
        [&self.opreturn]
    }
}

impl<T> Add for ByUnspendableType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            opreturn: self.opreturn + rhs.opreturn,
        }
    }
}

impl<T> AddAssign for ByUnspendableType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.opreturn += rhs.opreturn;
    }
}

impl<T: IVecs> IVecs for ByUnspendableType<T> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [("opreturn", &self.opreturn)]
                .into_iter()
                .map(|(name, field)| (name.to_string(), field.to_tree_node()))
                .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        self.opreturn.iter()
    }
}
