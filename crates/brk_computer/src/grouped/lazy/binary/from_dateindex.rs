use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, MonthIndex, QuarterIndex, SemesterIndex, Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{AnyExportableVec, BinaryTransform, IterableCloneableVec, LazyVecFrom2};

use crate::grouped::{ComputedVecValue, ComputedVecsFromDateIndex, ComputedVecsFromHeight, LazyTransform2Builder};

const VERSION: Version = Version::ZERO;

/// Lazy binary transform from two `ComputedVecsFromDateIndex` sources.
#[derive(Clone)]
pub struct LazyVecsFrom2FromDateIndex<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: Option<LazyVecFrom2<DateIndex, T, DateIndex, S1T, DateIndex, S2T>>,
    pub weekindex: LazyTransform2Builder<WeekIndex, T, S1T, S2T>,
    pub monthindex: LazyTransform2Builder<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2Builder<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2Builder<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2Builder<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2Builder<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyVecsFrom2FromDateIndex<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create from two `ComputedVecsFromDateIndex` sources.
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedVecsFromDateIndex<S1T>,
        source2: &ComputedVecsFromDateIndex<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: source1
                .dateindex
                .as_ref()
                .zip(source2.dateindex.as_ref())
                .map(|(s1, s2)| {
                    LazyVecFrom2::transformed::<F>(name, v, s1.boxed_clone(), s2.boxed_clone())
                }),
            weekindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }

    /// Create from a `ComputedVecsFromHeight` (first source) and `ComputedVecsFromDateIndex` (second source).
    /// Used for computing USD values from price (Height-based) and ratio (DateIndex-based).
    pub fn from_height_and_dateindex<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        source1: &ComputedVecsFromHeight<S1T>,
        source2: &ComputedVecsFromDateIndex<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            dateindex: source2.dateindex.as_ref().map(|s2| {
                LazyVecFrom2::transformed::<F>(
                    name,
                    v,
                    source1.dateindex.unwrap_last().boxed_clone(),
                    s2.boxed_clone(),
                )
            }),
            weekindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            monthindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.monthindex,
                &source2.monthindex,
            ),
            quarterindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.quarterindex,
                &source2.quarterindex,
            ),
            semesterindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.semesterindex,
                &source2.semesterindex,
            ),
            yearindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.yearindex,
                &source2.yearindex,
            ),
            decadeindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.decadeindex,
                &source2.decadeindex,
            ),
        }
    }
}

impl<T, S1T, S2T> Traversable for LazyVecsFrom2FromDateIndex<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    fn to_tree_node(&self) -> brk_traversable::TreeNode {
        brk_traversable::TreeNode::Branch(
            [
                self.dateindex
                    .as_ref()
                    .map(|v| ("dateindex".to_string(), v.to_tree_node())),
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
        let mut iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(std::iter::empty());
        if let Some(ref v) = self.dateindex {
            iter = Box::new(iter.chain(v.iter_any_exportable()));
        }
        iter = Box::new(iter.chain(self.weekindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.monthindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.quarterindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.semesterindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.yearindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.decadeindex.iter_any_exportable()));
        iter
    }
}
