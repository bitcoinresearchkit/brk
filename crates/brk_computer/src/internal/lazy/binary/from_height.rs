use brk_traversable::Traversable;
use brk_types::{
    DateIndex, DecadeIndex, DifficultyEpoch, Height, MonthIndex, QuarterIndex, SemesterIndex,
    Version, WeekIndex, YearIndex,
};
use schemars::JsonSchema;
use vecdb::{AnyExportableVec, BinaryTransform, IterableBoxedVec, LazyVecFrom2};

use crate::internal::{
    ComputedVecValue, ComputedVecsFromHeight, ComputedVecsFromTxindex, LazyTransform2Builder,
};

const VERSION: Version = Version::ZERO;

/// Lazy binary transform from two `ComputedVecsFromHeight` sources.
#[derive(Clone)]
pub struct LazyVecsFrom2FromHeight<T, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    pub height_extra: LazyTransform2Builder<Height, T, S1T, S2T>,
    pub dateindex: LazyTransform2Builder<DateIndex, T, S1T, S2T>,
    pub weekindex: LazyTransform2Builder<WeekIndex, T, S1T, S2T>,
    pub difficultyepoch: LazyTransform2Builder<DifficultyEpoch, T, S1T, S2T>,
    pub monthindex: LazyTransform2Builder<MonthIndex, T, S1T, S2T>,
    pub quarterindex: LazyTransform2Builder<QuarterIndex, T, S1T, S2T>,
    pub semesterindex: LazyTransform2Builder<SemesterIndex, T, S1T, S2T>,
    pub yearindex: LazyTransform2Builder<YearIndex, T, S1T, S2T>,
    pub decadeindex: LazyTransform2Builder<DecadeIndex, T, S1T, S2T>,
}

impl<T, S1T, S2T> LazyVecsFrom2FromHeight<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema + 'static,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    /// Create from two `ComputedVecsFromHeight` sources with explicit height sources.
    pub fn from_computed<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedVecsFromHeight<S1T>,
        source2: &ComputedVecsFromHeight<S2T>,
    ) -> Self {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            height_extra: LazyTransform2Builder::from_eager::<F>(
                name,
                v,
                &source1.height_extra,
                &source2.height_extra,
            ),
            dateindex: LazyTransform2Builder::from_eager::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            difficultyepoch: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.difficultyepoch,
                &source2.difficultyepoch,
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

    /// Create from a `ComputedVecsFromHeight` and a `ComputedVecsFromTxindex`.
    /// Used for ratios like type_count / total_output_count where the denominator
    /// comes from txindex-aggregated data.
    pub fn from_height_and_txindex<F: BinaryTransform<S1T, S2T, T>>(
        name: &str,
        version: Version,
        height_source1: IterableBoxedVec<Height, S1T>,
        height_source2: IterableBoxedVec<Height, S2T>,
        source1: &ComputedVecsFromHeight<S1T>,
        source2: &ComputedVecsFromTxindex<S2T>,
    ) -> Self
    where
        S2T: Ord + From<f64> + 'static,
        f64: From<S2T>,
    {
        let v = version + VERSION;

        Self {
            height: LazyVecFrom2::transformed::<F>(name, v, height_source1, height_source2),
            // For height_extra, source2 uses .height (EagerVecsBuilder) instead of .height_extra
            height_extra: LazyTransform2Builder::from_eager::<F>(
                name,
                v,
                &source1.height_extra,
                &source2.height,
            ),
            dateindex: LazyTransform2Builder::from_eager::<F>(
                name,
                v,
                &source1.dateindex,
                &source2.dateindex,
            ),
            weekindex: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.weekindex,
                &source2.weekindex,
            ),
            difficultyepoch: LazyTransform2Builder::from_lazy::<F, _, _, _, _>(
                name,
                v,
                &source1.difficultyepoch,
                &source2.difficultyepoch,
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

impl<T, S1T, S2T> Traversable for LazyVecsFrom2FromHeight<T, S1T, S2T>
where
    T: ComputedVecValue + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
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
        let mut iter: Box<dyn Iterator<Item = &dyn AnyExportableVec>> =
            Box::new(self.height.iter_any_exportable());
        iter = Box::new(iter.chain(self.height_extra.iter_any_exportable()));
        iter = Box::new(iter.chain(self.dateindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.weekindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.difficultyepoch.iter_any_exportable()));
        iter = Box::new(iter.chain(self.monthindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.quarterindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.semesterindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.yearindex.iter_any_exportable()));
        iter = Box::new(iter.chain(self.decadeindex.iter_any_exportable()));
        iter
    }
}
