mod last;
mod repeat;
mod strategy;

#[cfg(test)]
mod tests;

use std::{convert::Infallible, iter, marker::PhantomData, sync::Arc};

use bitview_traversable::{Index, SeriesLeaf, SeriesLeafWithSchema, Traversable, TreeNode};
use brk_types::{Day1, Version};
use schemars::SchemaGenerator;
use serde_json::to_value;
use vecdb::{
    AnyExportableVec, AnyVec, Cursor, READ_CHUNK_SIZE, ReadableBoxedVec, ReadableCloneableVec,
    ReadableVec, TypedVec, VecIndex, VecValue, short_type_name,
};

use crate::DailyValue;

pub use last::LastDay;
pub use repeat::RepeatDay;
pub use strategy::DayStrategy;

pub struct DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
{
    name: Arc<str>,
    version: Version,
    source: ReadableBoxedVec<Day1, T>,
    mapping: ReadableBoxedVec<I, Day1>,
    _phantom: PhantomData<fn() -> S>,
}

impl<I, T, S> Clone for DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version,
            source: self.source.clone(),
            mapping: self.mapping.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<I, T, S> DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: DayStrategy,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Day1, T> + ?Sized),
        mapping: &(impl ReadableCloneableVec<I, Day1> + ?Sized),
    ) -> Self {
        Self {
            name: Arc::from(name),
            version,
            source: source.read_only_boxed_clone(),
            mapping: mapping.read_only_boxed_clone(),
            _phantom: PhantomData,
        }
    }

    fn try_fold_values<B, E, F>(&self, from: usize, to: usize, init: B, f: F) -> Result<B, E>
    where
        F: FnMut(B, Option<T>) -> Result<B, E>,
    {
        let mapping_len = self.mapping.len();
        let to = to.min(mapping_len);
        if from >= to {
            return Ok(init);
        }

        if S::REPEATS_DAY {
            self.try_fold_repeated(from, to, init, f)
        } else {
            self.try_fold_sparse(from, to, mapping_len, init, f)
        }
    }

    fn try_fold_repeated<B, E, F>(&self, from: usize, to: usize, init: B, mut f: F) -> Result<B, E>
    where
        F: FnMut(B, Option<T>) -> Result<B, E>,
    {
        self.with_repeated_inputs(from, to, |mapping, start, values| {
            Self::repeated_values(mapping, start, values).try_fold(init, &mut f)
        })
    }

    /// Read the mapping once, then only the daily values it spans. Complete both
    /// reads before lending output chunks, since inputs can share publication locks.
    fn with_repeated_inputs<R>(
        &self,
        from: usize,
        to: usize,
        visit: impl FnOnce(&[Day1], usize, &[T]) -> R,
    ) -> R {
        let mapping = self.mapping.collect_range_dyn(from, to);
        let source_len = self.source.visible_len();
        let start = mapping
            .first()
            .map(|day| day.to_usize())
            .unwrap_or(source_len)
            .min(source_len);
        let end = mapping
            .last()
            .map(|day| day.to_usize().saturating_add(1))
            .unwrap_or(start)
            .min(source_len);
        let values = self.source.collect_range_dyn(start, end);
        visit(&mapping, start, &values)
    }

    fn repeated_values<'a>(
        mapping: &'a [Day1],
        start: usize,
        values: &'a [T],
    ) -> impl Iterator<Item = Option<T>> + 'a {
        mapping.iter().map(move |day| {
            day.to_usize()
                .checked_sub(start)
                .and_then(|i| values.get(i))
                .cloned()
        })
    }

    fn try_fold_sparse<B, E, F>(
        &self,
        from: usize,
        to: usize,
        mapping_len: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E>
    where
        F: FnMut(B, Option<T>) -> Result<B, E>,
    {
        let mapping = self
            .mapping
            .collect_range_dyn(from, S::mapping_end(to, mapping_len));
        let source_len = self.source.visible_len();
        let mut indices = Vec::with_capacity(to - from);
        let mut slots: Vec<Option<u32>> = Vec::with_capacity(to - from);

        for output_index in 0..(to - from) {
            let Some(source_index) = S::source_index(&mapping, output_index, source_len) else {
                slots.push(None);
                continue;
            };

            let slot = match indices.last() {
                Some(&last) if last == source_index => indices.len() - 1,
                Some(&last) => {
                    debug_assert!(last < source_index);
                    indices.push(source_index);
                    indices.len() - 1
                }
                None => {
                    indices.push(source_index);
                    0
                }
            };
            debug_assert!(u32::try_from(slot).is_ok());
            slots.push(Some(slot as u32));
        }

        let values = self.source.read_sorted_at(&indices);
        slots.into_iter().try_fold(init, |acc, slot| match slot {
            Some(slot) => f(acc, Some(values[slot as usize].clone())),
            None => f(acc, None),
        })
    }

    fn fold_values<B, F>(&self, from: usize, to: usize, init: B, mut f: F) -> B
    where
        F: FnMut(B, Option<T>) -> B,
    {
        match self.try_fold_values(from, to, init, |acc, value| {
            Ok::<_, Infallible>(f(acc, value))
        }) {
            Ok(result) => result,
            Err(error) => match error {},
        }
    }
}

impl<I, T, S> AnyVec for DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: DayStrategy,
{
    fn version(&self) -> Version {
        self.version + self.source.version() + self.mapping.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.mapping.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        vec![]
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<Option<T>>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<Option<T>>()
    }
}

impl<I, T, S> TypedVec for DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: DayStrategy,
{
    type I = I;
    type T = Option<T>;
}

impl<I, T, S> ReadableVec<I, Option<T>> for DailyView<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: DayStrategy,
{
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<Option<T>>) {
        let to = to.min(self.mapping.len());
        if from >= to {
            return;
        }
        buf.reserve(to - from);
        if S::REPEATS_DAY {
            self.with_repeated_inputs(from, to, |mapping, start, values| {
                buf.extend(Self::repeated_values(mapping, start, values));
            });
        } else {
            self.fold_values(from, to, (), |(), value| buf.push(value));
        }
    }

    fn cursor_chunk_size(&self) -> usize {
        self.mapping.cursor_chunk_size()
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[Option<T>])) {
        let to = to.min(self.mapping.len());
        if from >= to {
            return;
        }
        let size = READ_CHUNK_SIZE;
        let mut output = Vec::new();
        if S::REPEATS_DAY {
            self.with_repeated_inputs(from, to, |mapping, start, values| {
                for (chunk, mapping) in mapping.chunks(size).enumerate() {
                    output.clear();
                    output.extend(Self::repeated_values(mapping, start, values));
                    f(from + chunk * size, &output);
                }
            });
        } else {
            let mut at = from;
            self.fold_values(from, to, (), |(), value| {
                output.push(value);
                if output.len() == size {
                    f(at, &output);
                    at += output.len();
                    output.clear();
                }
            });
            if !output.is_empty() {
                f(at, &output);
            }
        }
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(Option<T>)) {
        self.fold_values(from, to, (), |(), value| f(value));
    }

    fn fold_range_at<B, F: FnMut(B, Option<T>) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> B {
        self.fold_values(from, to, init, f)
    }

    fn try_fold_range_at<B, E, F: FnMut(B, Option<T>) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        self.try_fold_values(from, to, init, f)
    }

    fn collect_one_at(&self, index: usize) -> Option<Option<T>> {
        let mapping_len = self.mapping.len();
        if index >= mapping_len {
            return None;
        }

        let mapping = self
            .mapping
            .collect_range_dyn(index, S::mapping_end(index.saturating_add(1), mapping_len));
        Some(
            S::source_index(&mapping, 0, self.source.visible_len())
                .and_then(|day| self.source.collect_one_at(day)),
        )
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<Option<T>>) {
        if indices.is_empty() {
            return;
        }
        let mapping_len = self.mapping.len();
        let indices = &indices[..indices.partition_point(|&i| i < mapping_len)];
        if indices.is_empty() {
            return;
        }
        if let &[index] = indices {
            if let Some(value) = self.collect_one_at(index) {
                out.push(value);
            }
            return;
        }
        if indices.windows(2).all(|pair| pair[1] == pair[0] + 1) {
            self.read_into_at(indices[0], indices[indices.len() - 1] + 1, out);
            return;
        }
        let source_len = self.source.visible_len();
        let mut mapping = Cursor::new(&*self.mapping);
        let mut window = Vec::with_capacity(2);
        let mut requested = Vec::with_capacity(indices.len());
        let mut slots = Vec::with_capacity(indices.len());
        for &index in indices {
            window.clear();
            for i in index..S::mapping_end(index + 1, mapping_len) {
                if let Some(day) = mapping.get(i) {
                    window.push(day);
                }
            }
            if window.is_empty() {
                continue;
            }
            slots.push(S::source_index(&window, 0, source_len).map(|i| {
                if requested.last() != Some(&i) {
                    requested.push(i);
                }
                requested.len() - 1
            }));
        }
        let values = self.source.read_sorted_at(&requested);
        out.extend(
            slots
                .into_iter()
                .map(|slot| slot.map(|slot| values[slot].clone())),
        );
    }
}

impl<I, T, S> Traversable for DailyView<I, T, S>
where
    I: VecIndex,
    T: DailyValue,
    S: DayStrategy,
{
    fn to_tree_node(&self) -> TreeNode {
        let indexes = Index::try_from(I::to_string()).ok().into_iter().collect();
        let leaf = SeriesLeaf::new(
            self.name().to_string(),
            self.value_type_to_string().to_string(),
            indexes,
        );
        let schema = SchemaGenerator::default().into_root_schema_for::<Option<T>>();
        let schema_json = to_value(schema).unwrap_or_default();

        TreeNode::Leaf(SeriesLeafWithSchema::new(leaf, schema_json))
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        iter::once(self as &dyn AnyExportableVec)
    }
}
