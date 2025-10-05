use std::collections::HashMap;

#[cfg(feature = "derive")]
pub use brk_vecs_derive::IVecs;
use serde::Serialize;
use vecdb::AnyCollectableVec;

pub trait IVecs {
    fn to_tree_node(&self) -> TreeNode;
    fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec>;
}

// Terminal implementation for any type that implements AnyCollectableVec
// impl<T: AnyCollectableVec> IVecs for T {
//     fn to_tree_node(&self) -> TreeNode {
//         TreeNode::Leaf(self.name().to_string())
//     }

//     fn iter<'a>(
//         &'a self,
//     ) -> Box<dyn Iterator<Item = &'a dyn AnyCollectableVec> + 'a> {
//         Box::new(std::iter::once(self as &dyn AnyCollectableVec))
//     }
// }

// For Option types
// impl<T: IVecs> IVecs for Option<T> {
//     fn to_tree_node(&self) -> TreeNode {
//         match self {
//             Some(inner) => inner.to_tree_node(),
//             None => TreeNode::Branch(HashMap::new()),
//         }
//     }

//     fn iter<'a>(
//         &'a self,
//     ) -> Box<dyn Iterator<Item = &'a dyn AnyCollectableVec> + 'a> {
//         match self {
//             Some(inner) => inner.iter(),
//             None => Box::new(std::iter::empty()),
//         }
//     }
// }

// For Box types
// impl<T: IVecs> IVecs for Box<T> {
//     fn to_tree_node(&self) -> TreeNode {
//         (**self).to_tree_node()
//     }

//     fn iter<'a>(
//         &'a self,
//     ) -> Box<dyn Iterator<Item = &'a dyn AnyCollectableVec> + 'a> {
//         (**self).iter()
//     }
// }

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TreeNode {
    Branch(HashMap<String, TreeNode>),
    Leaf(String),
}
