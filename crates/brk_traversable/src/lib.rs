use std::{collections::BTreeMap, fmt::Debug};

pub use brk_types::TreeNode;

#[cfg(feature = "derive")]
pub use brk_traversable_derive::Traversable;
use vecdb::{
    AnyVec, AnyWritableVec, CompressedVec, ComputedVec, EagerVec, Formattable, LazyVecFrom1,
    LazyVecFrom2, LazyVecFrom3, Pco, RawVec, StoredVec, VecIndex, VecValue,
};

pub trait Traversable {
    fn to_tree_node(&self) -> TreeNode;
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec>;
}

impl<I, T> Traversable for RawVec<I, T>
where
    I: VecIndex,
    T: VecValue + Formattable,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T> Traversable for CompressedVec<I, T>
where
    I: VecIndex,
    T: Pco + Formattable,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T> Traversable for StoredVec<I, T>
where
    I: VecIndex,
    T: Pco + Formattable,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T> Traversable for EagerVec<I, T>
where
    I: VecIndex,
    T: Pco + Formattable,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T, S1I, S1T> Traversable for LazyVecFrom1<I, T, S1I, S1T>
where
    I: VecIndex,
    T: VecValue + Formattable,
    S1I: VecIndex,
    S1T: VecValue,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T, S1I, S1T, S2I, S2T> Traversable for LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: VecIndex,
    T: VecValue + Formattable,
    S1I: VecIndex,
    S1T: VecValue,
    S2I: VecIndex,
    S2T: VecValue,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> Traversable
    for LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: VecIndex,
    T: VecValue + Formattable,
    S1I: VecIndex,
    S1T: VecValue,
    S2I: VecIndex,
    S2T: VecValue,
    S3I: VecIndex,
    S3T: VecValue,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> Traversable
    for ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: VecIndex,
    T: Pco + Formattable,
    S1I: VecIndex,
    S1T: Pco,
    S2I: VecIndex,
    S2T: Pco,
    S3I: VecIndex,
    S3T: Pco,
{
    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        std::iter::once(self as &dyn AnyWritableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Leaf(self.name().to_string())
    }
}

impl<T: Traversable + ?Sized> Traversable for Box<T> {
    fn to_tree_node(&self) -> TreeNode {
        (**self).to_tree_node()
    }

    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        (**self).iter_any_writable()
    }
}

impl<T: Traversable> Traversable for Option<T> {
    fn to_tree_node(&self) -> TreeNode {
        match self {
            Some(inner) => inner.to_tree_node(),
            None => TreeNode::Branch(BTreeMap::new()),
        }
    }

    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        match self {
            Some(inner) => {
                Box::new(inner.iter_any_writable()) as Box<dyn Iterator<Item = &dyn AnyWritableVec>>
            }
            None => Box::new(std::iter::empty()),
        }
    }
}

impl<K: Debug, V: Traversable> Traversable for BTreeMap<K, V> {
    fn to_tree_node(&self) -> TreeNode {
        let children = self
            .iter()
            .map(|(k, v)| (format!("{:?}", k), v.to_tree_node()))
            .collect();
        TreeNode::Branch(children)
    }

    fn iter_any_writable(&self) -> impl Iterator<Item = &dyn AnyWritableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyWritableVec>> = Box::new(std::iter::empty());
        for v in self.values() {
            iter = Box::new(iter.chain(v.iter_any_writable()));
        }
        iter
    }
}
