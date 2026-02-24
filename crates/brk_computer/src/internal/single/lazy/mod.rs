//! Lazy aggregation primitives (finer index → coarser index).
//!
//! These types implement GROUP BY: map each coarser output index to a range
//! in the finer source, then aggregate that range. They implement the vecdb
//! ReadableVec trait directly.

/// Common trait implementations for lazy aggregation types.
///
/// Provides: Clone, AnyVec, TypedVec, ReadableVec, Traversable.
/// The struct must have fields: name, version, source, mapping, for_each_range.
macro_rules! impl_lazy_agg {
    ($name:ident) => {
        impl<I, T, S1I, S2T> Clone for $name<I, T, S1I, S2T>
        where
            I: VecIndex,
            T: ComputedVecValue + JsonSchema,
            S1I: VecIndex,
            S2T: VecValue,
        {
            fn clone(&self) -> Self {
                Self {
                    name: self.name.clone(),
                    version: self.version,
                    source: self.source.clone(),
                    mapping: self.mapping.clone(),
                    for_each_range: self.for_each_range,
                }
            }
        }

        impl<I, T, S1I, S2T> vecdb::AnyVec for $name<I, T, S1I, S2T>
        where
            I: VecIndex,
            T: ComputedVecValue + JsonSchema,
            S1I: VecIndex,
            S2T: VecValue,
        {
            fn version(&self) -> Version {
                self.version + self.source.version() + self.mapping.version()
            }
            fn name(&self) -> &str {
                &self.name
            }
            fn index_type_to_string(&self) -> &'static str {
                I::to_string()
            }
            fn len(&self) -> usize {
                self.mapping.len()
            }
            #[inline]
            fn value_type_to_size_of(&self) -> usize {
                size_of::<T>()
            }
            #[inline]
            fn value_type_to_string(&self) -> &'static str {
                vecdb::short_type_name::<T>()
            }
            #[inline]
            fn region_names(&self) -> Vec<String> {
                vec![]
            }
        }

        impl<I, T, S1I, S2T> vecdb::TypedVec for $name<I, T, S1I, S2T>
        where
            I: VecIndex,
            T: ComputedVecValue + JsonSchema,
            S1I: VecIndex,
            S2T: VecValue,
        {
            type I = I;
            type T = T;
        }

        impl<I, T, S1I, S2T> vecdb::ReadableVec<I, T> for $name<I, T, S1I, S2T>
        where
            I: VecIndex,
            T: ComputedVecValue + JsonSchema,
            S1I: VecIndex,
            S2T: VecValue,
        {
            fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
                let to = to.min(self.mapping.len());
                if from >= to { return; }
                buf.reserve(to - from);
                (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| buf.push(v));
            }

            fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
                let to = to.min(self.mapping.len());
                if from >= to { return; }
                (self.for_each_range)(from, to, &self.source, &self.mapping, f);
            }

            #[inline]
            fn fold_range_at<B, F: FnMut(B, T) -> B>(
                &self,
                from: usize,
                to: usize,
                init: B,
                mut f: F,
            ) -> B
            where
                Self: Sized,
            {
                let to = to.min(self.mapping.len());
                if from >= to { return init; }
                let mut acc = Some(init);
                (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| {
                    acc = Some(f(acc.take().unwrap(), v));
                });
                acc.unwrap()
            }

            #[inline]
            fn try_fold_range_at<B, E, F: FnMut(B, T) -> std::result::Result<B, E>>(
                &self,
                from: usize,
                to: usize,
                init: B,
                mut f: F,
            ) -> std::result::Result<B, E>
            where
                Self: Sized,
            {
                let to = to.min(self.mapping.len());
                if from >= to { return Ok(init); }
                let mut acc: Option<std::result::Result<B, E>> = Some(Ok(init));
                (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| {
                    if let Some(Ok(a)) = acc.take() {
                        acc = Some(f(a, v));
                    }
                });
                acc.unwrap()
            }

            #[inline]
            fn collect_one_at(&self, index: usize) -> Option<T> {
                if index >= self.mapping.len() {
                    return None;
                }
                let mut result = None;
                (self.for_each_range)(index, index + 1, &self.source, &self.mapping, &mut |v| result = Some(v));
                result
            }
        }

        impl<I, T, S1I, S2T> Traversable for $name<I, T, S1I, S2T>
        where
            I: VecIndex,
            T: ComputedVecValue + JsonSchema + 'static,
            S1I: VecIndex + 'static,
            S2T: VecValue,
        {
            fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
                std::iter::once(self as &dyn vecdb::AnyExportableVec)
            }

            fn to_tree_node(&self) -> brk_types::TreeNode {
                use vecdb::AnyVec;
                let index_str = I::to_string();
                let index = brk_types::Index::try_from(index_str).ok();
                let indexes = index.into_iter().collect();
                let leaf = brk_types::MetricLeaf::new(
                    self.name().to_string(),
                    self.value_type_to_string().to_string(),
                    indexes,
                );
                let schema =
                    schemars::SchemaGenerator::default().into_root_schema_for::<T>();
                let schema_json = serde_json::to_value(schema).unwrap_or_default();
                brk_types::TreeNode::Leaf(brk_types::MetricLeafWithSchema::new(
                    leaf,
                    schema_json,
                ))
            }
        }
    };
}

mod average;
mod cumulative;
mod distribution;
mod full;
mod last;
mod max;
mod min;
mod percentile;
mod percentiles;
mod sparse_last;
mod sum;
mod sum_cum;

pub use average::*;
pub use cumulative::*;
pub use distribution::*;
pub use full::*;
pub use last::*;
pub use max::*;
pub use min::*;
pub use percentile::*;
pub use percentiles::*;
pub use sparse_last::*;
pub use sum::*;
pub use sum_cum::*;
