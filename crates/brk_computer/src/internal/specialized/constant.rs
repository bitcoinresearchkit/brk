use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, Height, MonthIndex, QuarterIndex, SemesterIndex, TreeNode, Version,
    WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{
    AnyExportableVec, Formattable, IterableCloneableVec, LazyVecFrom1, UnaryTransform, VecValue,
};

use crate::{indexes, internal::ComputedVecValue};

/// Lazy constant vecs for all index levels.
/// Uses const generic transforms to return the same value for every index.
#[derive(Clone)]
pub struct ConstantVecs<T>
where
    T: VecValue + Formattable + Serialize + JsonSchema,
{
    pub height: LazyVecFrom1<Height, T, Height, Height>,
    pub dateindex: LazyVecFrom1<DateIndex, T, DateIndex, DateIndex>,
    pub weekindex: LazyVecFrom1<WeekIndex, T, WeekIndex, WeekIndex>,
    pub monthindex: LazyVecFrom1<MonthIndex, T, MonthIndex, MonthIndex>,
    pub quarterindex: LazyVecFrom1<QuarterIndex, T, QuarterIndex, QuarterIndex>,
    pub semesterindex: LazyVecFrom1<SemesterIndex, T, SemesterIndex, SemesterIndex>,
    pub yearindex: LazyVecFrom1<YearIndex, T, YearIndex, YearIndex>,
    pub decadeindex: LazyVecFrom1<DecadeIndex, T, DecadeIndex, DecadeIndex>,
}

impl<T: VecValue + Formattable + Serialize + JsonSchema> ConstantVecs<T> {
    /// Create constant vecs using a transform that ignores input and returns a constant.
    pub fn new<F>(name: &str, version: Version, indexes: &indexes::Vecs) -> Self
    where
        F: UnaryTransform<Height, T>
            + UnaryTransform<DateIndex, T>
            + UnaryTransform<WeekIndex, T>
            + UnaryTransform<MonthIndex, T>
            + UnaryTransform<QuarterIndex, T>
            + UnaryTransform<SemesterIndex, T>
            + UnaryTransform<YearIndex, T>
            + UnaryTransform<DecadeIndex, T>,
    {
        Self {
            height: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.block.height_to_height.boxed_clone(),
            ),
            dateindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.dateindex_to_dateindex.boxed_clone(),
            ),
            weekindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.weekindex_to_weekindex.boxed_clone(),
            ),
            monthindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.monthindex_to_monthindex.boxed_clone(),
            ),
            quarterindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.quarterindex_to_quarterindex.boxed_clone(),
            ),
            semesterindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.semesterindex_to_semesterindex.boxed_clone(),
            ),
            yearindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.yearindex_to_yearindex.boxed_clone(),
            ),
            decadeindex: LazyVecFrom1::transformed::<F>(
                name,
                version,
                indexes.time.decadeindex_to_decadeindex.boxed_clone(),
            ),
        }
    }
}

impl<T> Traversable for ConstantVecs<T>
where
    T: ComputedVecValue + JsonSchema,
{
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            [
                Some(("height".to_string(), self.height.to_tree_node())),
                Some(("dateindex".to_string(), self.dateindex.to_tree_node())),
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
            Box::new(self.height.iter_any_exportable());
        regular_iter = Box::new(regular_iter.chain(self.dateindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.weekindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.monthindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.quarterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.semesterindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.yearindex.iter_any_exportable()));
        regular_iter = Box::new(regular_iter.chain(self.decadeindex.iter_any_exportable()));
        regular_iter
    }
}
