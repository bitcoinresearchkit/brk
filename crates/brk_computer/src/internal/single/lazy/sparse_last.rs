//! Sparse last-value aggregation for time-based periods.
//!
//! Unlike [`LazyLast`], which skips empty periods, `SparseLast` produces
//! `Option<T>` for every period slot: `Some(v)` when blocks exist, `None`
//! when a time period contains no blocks. This preserves dense positional
//! mapping (slot i = period start + i) required for correct serialization.

use std::sync::Arc;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Cursor, ReadableBoxedVec, VecIndex};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

/// Lazy last-value aggregation that emits `Option<T>` for every time period.
///
/// For periods containing blocks: `Some(last_value_in_period)`.
/// For empty periods (no blocks mined): `None`.
pub struct SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    name: Arc<str>,
    version: Version,
    source: ReadableBoxedVec<Height, T>,
    first_height: ReadableBoxedVec<I, Height>,
}

impl<I, T> SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn new(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        Self {
            name: Arc::from(name),
            version: version + VERSION,
            source,
            first_height,
        }
    }

    pub fn height_source(&self) -> &ReadableBoxedVec<Height, T> {
        &self.source
    }

    pub fn first_height(&self) -> &ReadableBoxedVec<I, Height> {
        &self.first_height
    }

    /// Dense iteration: calls `f` for every period in `[from, to)`,
    /// including empty ones (with `None`).
    fn for_each_impl(
        from: usize,
        to: usize,
        source: &ReadableBoxedVec<Height, T>,
        first_height: &ReadableBoxedVec<I, Height>,
        f: &mut dyn FnMut(Option<T>),
    ) {
        let map_end = (to + 1).min(first_height.len());
        let heights = first_height.collect_range_dyn(from, map_end);
        let source_len = source.len();
        let mut cursor = Cursor::from_dyn(&**source);

        for idx in 0..(to - from) {
            let current_first = heights[idx].to_usize();
            let next_first = heights
                .get(idx + 1)
                .map(|h| h.to_usize())
                .unwrap_or(source_len);

            // Empty period: no blocks belong to this time slot
            if next_first == 0 || current_first >= next_first {
                f(None);
                continue;
            }

            // Last height in this period
            let target = next_first - 1;

            if cursor.position() <= target {
                cursor.advance(target - cursor.position());
                match cursor.next() {
                    Some(v) => f(Some(v)),
                    None => f(None),
                }
            } else {
                match source.collect_one_at(target) {
                    Some(v) => f(Some(v)),
                    None => f(None),
                }
            }
        }
    }
}

impl<I, T> Clone for SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version,
            source: self.source.clone(),
            first_height: self.first_height.clone(),
        }
    }
}

impl<I, T> vecdb::AnyVec for SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    fn version(&self) -> Version {
        self.version + self.source.version() + self.first_height.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    fn len(&self) -> usize {
        self.first_height.len()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<Option<T>>()
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

impl<I, T> vecdb::TypedVec for SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    type I = I;
    type T = Option<T>;
}

impl<I, T> vecdb::ReadableVec<I, Option<T>> for SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
{
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<Option<T>>) {
        let to = to.min(self.first_height.len());
        if from >= to {
            return;
        }
        buf.reserve(to - from);
        Self::for_each_impl(from, to, &self.source, &self.first_height, &mut |v| {
            buf.push(v)
        });
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(Option<T>)) {
        let to = to.min(self.first_height.len());
        if from >= to {
            return;
        }
        Self::for_each_impl(from, to, &self.source, &self.first_height, f);
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, Option<T>) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B
    where
        Self: Sized,
    {
        let to = to.min(self.first_height.len());
        if from >= to {
            return init;
        }
        let mut acc = Some(init);
        Self::for_each_impl(from, to, &self.source, &self.first_height, &mut |v| {
            acc = Some(f(acc.take().unwrap(), v));
        });
        acc.unwrap()
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, Option<T>) -> std::result::Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> std::result::Result<B, E>
    where
        Self: Sized,
    {
        let to = to.min(self.first_height.len());
        if from >= to {
            return Ok(init);
        }
        let mut acc: Option<std::result::Result<B, E>> = Some(Ok(init));
        Self::for_each_impl(from, to, &self.source, &self.first_height, &mut |v| {
            if let Some(Ok(a)) = acc.take() {
                acc = Some(f(a, v));
            }
        });
        acc.unwrap()
    }

    #[inline]
    fn collect_one_at(&self, index: usize) -> Option<Option<T>> {
        if index >= self.first_height.len() {
            return None;
        }
        let current_first = self.first_height.collect_one_at(index)?.to_usize();
        let next_first = self
            .first_height
            .collect_one_at(index + 1)
            .map(|h| h.to_usize())
            .unwrap_or(self.source.len());
        if next_first == 0 || current_first >= next_first {
            return Some(None);
        }
        Some(self.source.collect_one_at(next_first - 1))
    }
}

impl<I, T> Traversable for SparseLast<I, T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
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
        let schema = schemars::SchemaGenerator::default().into_root_schema_for::<Option<T>>();
        let schema_json = serde_json::to_value(schema).unwrap_or_default();
        brk_types::TreeNode::Leaf(brk_types::MetricLeafWithSchema::new(leaf, schema_json))
    }
}
