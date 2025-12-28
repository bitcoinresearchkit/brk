use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Version};
use schemars::JsonSchema;
use vecdb::{AnyExportableVec, IterableBoxedVec, LazyVecFrom1, UnaryTransform};

use super::{ComputedVecValue, ComputedVecsFromHeightStrict, LazyTransformBuilder};

const VERSION: Version = Version::ZERO;

/// Fully lazy version of `ComputedVecsFromHeightStrict` where all vecs are lazy transforms.
#[derive(Clone)]
pub struct LazyVecsFromHeightStrict<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    pub difficultyepoch: LazyTransformBuilder<DifficultyEpoch, T, S1T>,
}

impl<T, S1T> LazyVecsFromHeightStrict<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy transform from a stored `ComputedVecsFromHeightStrict`.
    /// F is the transform type (e.g., `Negate`, `Halve`).
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &ComputedVecsFromHeightStrict<S1T>,
    ) -> Self {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            difficultyepoch: LazyTransformBuilder::from_eager::<F>(name, v, &source.difficultyepoch),
        }
    }
}

impl<T, S1T> Traversable for LazyVecsFromHeightStrict<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::Branch(
            [
                Some(("height".to_string(), self.height.to_tree_node())),
                Some((
                    "difficultyepoch".to_string(),
                    self.difficultyepoch.to_tree_node(),
                )),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
        .merge_branches()
        .unwrap()
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        let mut regular_iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(self.height.iter_any_exportable());
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_exportable()));
        regular_iter
    }
}
