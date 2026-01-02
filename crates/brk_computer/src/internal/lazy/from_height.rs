use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{AnyExportableVec, IterableBoxedVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{ComputedVecValue, ComputedVecsFromHeight, LazyTransformBuilder};

const VERSION: Version = Version::ZERO;

/// Fully lazy version of `ComputedVecsFromHeight` where all vecs are lazy transforms.
/// Each index uses `LazyTransformBuilder` sourced from its corresponding stored groups.
#[derive(Clone)]
pub struct LazyVecsFromHeight<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub height: LazyVecFrom1<Height, T, Height, S1T>,
    pub height_extra: LazyTransformBuilder<Height, T, S1T>,
    pub dateindex: LazyTransformBuilder<DateIndex, T, S1T>,
    pub weekindex: LazyTransformBuilder<WeekIndex, T, S1T>,
    pub difficultyepoch: LazyTransformBuilder<DifficultyEpoch, T, S1T>,
    pub monthindex: LazyTransformBuilder<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformBuilder<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformBuilder<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformBuilder<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformBuilder<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyVecsFromHeight<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy transform from a stored `ComputedVecsFromHeight`.
    /// F is the transform type (e.g., `Negate`, `Halve`).
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        height_source: IterableBoxedVec<Height, S1T>,
        source: &ComputedVecsFromHeight<S1T>,
    ) -> Self {
        let v = version + VERSION;
        Self {
            height: LazyVecFrom1::transformed::<F>(name, v, height_source),
            height_extra: LazyTransformBuilder::from_eager::<F>(name, v, &source.height_extra),
            dateindex: LazyTransformBuilder::from_eager::<F>(name, v, &source.dateindex),
            weekindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.weekindex),
            difficultyepoch: LazyTransformBuilder::from_lazy::<F, _, _>(
                name,
                v,
                &source.difficultyepoch,
            ),
            monthindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.monthindex),
            quarterindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.quarterindex),
            semesterindex: LazyTransformBuilder::from_lazy::<F, _, _>(
                name,
                v,
                &source.semesterindex,
            ),
            yearindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.yearindex),
            decadeindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.decadeindex),
        }
    }
}

impl<T, S1T> Traversable for LazyVecsFromHeight<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        let height_extra_node = self.height_extra.to_tree_node();
        brk_traversable::TreeNode::Branch(
            [
                Some(("height".to_string(), self.height.to_tree_node())),
                if height_extra_node.is_empty() {
                    None
                } else {
                    Some(("height_extra".to_string(), height_extra_node))
                },
                Some(("dateindex".to_string(), self.dateindex.to_tree_node())),
                Some(("weekindex".to_string(), self.weekindex.to_tree_node())),
                Some((
                    "difficultyepoch".to_string(),
                    self.difficultyepoch.to_tree_node(),
                )),
                Some(("monthindex".to_string(), self.monthindex.to_tree_node())),
                Some(("quarterindex".to_string(), self.quarterindex.to_tree_node())),
                Some((
                    "semesterindex".to_string(),
                    self.semesterindex.to_tree_node(),
                )),
                Some(("yearindex".to_string(), self.yearindex.to_tree_node())),
                Some(("decadeindex".to_string(), self.decadeindex.to_tree_node())),
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
        regular_iter = Box::new(regular_iter.chain(self.height_extra.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.dateindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.difficultyepoch.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_exportable()));
        regular_iter
    }
}
