use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{AnyExportableVec, IterableBoxedVec, LazyVecFrom1, UnaryTransform};

use crate::internal::{ComputedVecValue, ComputedVecsFromDateIndex, LazyTransformBuilder};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct LazyVecsFromDateIndex<T, S1T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
{
    pub dateindex: Option<LazyVecFrom1<DateIndex, T, DateIndex, S1T>>,
    pub dateindex_extra: LazyTransformBuilder<DateIndex, T, S1T>,
    pub weekindex: LazyTransformBuilder<WeekIndex, T, S1T>,
    pub monthindex: LazyTransformBuilder<MonthIndex, T, S1T>,
    pub quarterindex: LazyTransformBuilder<QuarterIndex, T, S1T>,
    pub semesterindex: LazyTransformBuilder<SemesterIndex, T, S1T>,
    pub yearindex: LazyTransformBuilder<YearIndex, T, S1T>,
    pub decadeindex: LazyTransformBuilder<DecadeIndex, T, S1T>,
}

impl<T, S1T> LazyVecsFromDateIndex<T, S1T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
{
    /// Create a lazy transform from a stored `ComputedVecsFromDateIndex`.
    /// F is the transform type (e.g., `Negate`, `Halve`).
    pub fn from_computed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        dateindex_source: Option<IterableBoxedVec<DateIndex, S1T>>,
        source: &ComputedVecsFromDateIndex<S1T>,
    ) -> Self {
        let v = version + VERSION;
        Self {
            dateindex: dateindex_source.map(|s| LazyVecFrom1::transformed::<F>(name, v, s)),
            dateindex_extra: LazyTransformBuilder::from_eager::<F>(
                name,
                v,
                &source.dateindex_extra,
            ),
            weekindex: LazyTransformBuilder::from_lazy::<F, _, _>(name, v, &source.weekindex),
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

impl<T, S1T> Traversable for LazyVecsFromDateIndex<T, S1T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        let dateindex_extra_node = self.dateindex_extra.to_tree_node();
        brk_traversable::TreeNode::Branch(
            [
                self.dateindex
                    .as_ref()
                    .map(|v| ("dateindex".to_string(), v.to_tree_node())),
                if dateindex_extra_node.is_empty() {
                    None
                } else {
                    Some(("dateindex_extra".to_string(), dateindex_extra_node))
                },
                Some(("weekindex".to_string(), self.weekindex.to_tree_node())),
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
            Box::new(std::iter::empty());
        if let Some(ref dateindex) = self.dateindex {
            regular_iter = Box::new(regular_iter.chain(dateindex.iter_any_exportable()));
        }
        regular_iter = Box::new(regular_iter.chain(self.dateindex_extra.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_exportable()));
        regular_iter
    }
}
