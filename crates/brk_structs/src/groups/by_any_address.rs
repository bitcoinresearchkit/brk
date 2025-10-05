use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

#[derive(Debug, Default)]
pub struct ByAnyAddress<T> {
    pub loaded: T,
    pub empty: T,
}

impl<T> ByAnyAddress<Option<T>> {
    pub fn take(&mut self) {
        self.loaded.take();
        self.empty.take();
    }
}

impl<T: IVecs> IVecs for ByAnyAddress<T> {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [("loaded", &self.loaded), ("empty", &self.empty)]
                .into_iter()
                .map(|(name, field)| (name.to_string(), field.to_tree_node()))
                .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(self.loaded.iter());
        iter = Box::new(iter.chain(self.empty.iter()));
        iter
    }
}
