use std::{marker::PhantomData, sync::Arc};

use crate::{
    AnyVec, READ_CHUNK_SIZE, ReadOnlyClone, ReadableVec, TypedVec, UnaryTransform, VecValue,
    Version, short_type_name,
};

use super::{ColumnId, ReadableColumnarVec};

/// Lazy scalar transformation that preserves the source's column structure.
pub struct LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    source: S,
    compute: fn(S::T) -> T,
    read_rows: fn(&S, usize, usize, &mut Vec<C::Row<T>>),
    visit_rows: fn(&S, usize, usize, &mut dyn FnMut(usize, &[C::Row<T>])),
    visit_columns: fn(&S, &[C], usize, usize, &mut dyn FnMut(C, usize, &[T])),
    columns: PhantomData<C>,
}

impl<S, T, C> Clone for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: Arc::clone(&self.name),
            base_version: self.base_version,
            source: self.source.clone(),
            compute: self.compute,
            read_rows: self.read_rows,
            visit_rows: self.visit_rows,
            visit_columns: self.visit_columns,
            columns: PhantomData,
        }
    }
}

impl<S, T, C> LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    /// Creates a single-source lazy scalar transformation while preserving columns.
    pub fn transformed<F>(name: &str, version: Version, source: S) -> Self
    where
        F: UnaryTransform<S::T, T>,
    {
        Self {
            name: Arc::from(name),
            base_version: version,
            source,
            compute: F::apply,
            read_rows: |source, from, to, out| {
                source.for_each_chunk_at(from, to, &mut |_, rows| {
                    out.extend(rows.iter().cloned().map(|row| C::map(row, F::apply)));
                });
            },
            visit_rows: |source, from, to, f| {
                let chunk_size = source.cursor_chunk_size().clamp(1, READ_CHUNK_SIZE);
                let mut output = Vec::new();
                source.for_each_chunk_at(from, to, &mut |at, rows| {
                    for (chunk, rows) in rows.chunks(chunk_size).enumerate() {
                        output.clear();
                        output.extend(rows.iter().cloned().map(|row| C::map(row, F::apply)));
                        f(at + chunk * chunk_size, &output);
                    }
                });
            },
            visit_columns: |source, columns, from, to, f| {
                let chunk_size = source.cursor_chunk_size().clamp(1, READ_CHUNK_SIZE);
                let mut output = Vec::new();
                source.for_each_column_chunk_at(columns, from, to, &mut |column, at, values| {
                    for (chunk, values) in values.chunks(chunk_size).enumerate() {
                        output.clear();
                        output.extend(values.iter().cloned().map(F::apply));
                        f(column, at + chunk * chunk_size, &output);
                    }
                });
            },
            columns: PhantomData,
        }
    }
}

impl<S, T, C> AnyVec for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    fn version(&self) -> Version {
        self.base_version.combine(self.source.version())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.source.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        self.source.index_type_to_string()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<C::Row<T>>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<C::Row<T>>()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<S, T, C> TypedVec for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    type I = S::I;
    type T = C::Row<T>;
}

impl<S, T, C> ReadableColumnarVec<C> for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    type I = S::I;
    type T = T;

    fn read_column_sorted_into_at(&self, column: C, indices: &[usize], out: &mut Vec<T>) {
        let len = self.len();
        let indices = &indices[..indices.partition_point(|&i| i < len)];
        if let (Some(&first), Some(&last)) = (indices.first(), indices.last())
            && last - first + 1 < indices.len()
        {
            // A dense duplicate-heavy request needs fewer transformations if
            // the requested span is evaluated once and then gathered.
            let mut values = Vec::with_capacity(last - first + 1);
            self.for_each_column_chunk_at(&[column], first, last + 1, &mut |_, _, chunk| {
                values.extend_from_slice(chunk);
            });
            out.extend(indices.iter().map(|&i| values[i - first].clone()));
            return;
        }
        let mut values = Vec::with_capacity(indices.len());
        self.source
            .read_column_sorted_into_at(column, indices, &mut values);
        out.extend(values.into_iter().map(self.compute));
    }

    fn for_each_column_chunk_at<F>(&self, columns: &[C], from: usize, to: usize, f: &mut F)
    where
        F: FnMut(C, usize, &[T]),
    {
        (self.visit_columns)(&self.source, columns, from, to, f);
    }

    fn for_each_column_sorted_at<F>(&self, columns: &[C], indices: &[usize], f: &mut F)
    where
        F: FnMut(C, &[T]),
    {
        let mut output = Vec::with_capacity(indices.len());
        self.source
            .for_each_column_sorted_at(columns, indices, &mut |column, values| {
                output.clear();
                output.extend(values.iter().cloned().map(self.compute));
                f(column, &output);
            });
    }
}

impl<S, T, C> ReadableVec<S::I, C::Row<T>> for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<C::Row<T>>) {
        let from = from.min(self.len());
        let to = to.min(self.len());
        if from >= to {
            return;
        }

        out.reserve(to - from);
        (self.read_rows)(&self.source, from, to, out);
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[C::Row<T>])) {
        (self.visit_rows)(&self.source, from, to, f);
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(C::Row<T>)) {
        self.for_each_chunk_at(from, to, &mut |_, rows| {
            rows.iter().cloned().for_each(&mut *f);
        });
    }

    fn fold_range_at<B, F: FnMut(B, C::Row<T>) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_chunk_at(from, to, &mut |_, rows| {
            acc = Some(rows.iter().cloned().fold(acc.take().unwrap(), &mut f));
        });
        acc.unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, C::Row<T>) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E> {
        let compute = self.compute;
        self.source
            .try_fold_range_at(from, to, init, |acc, row| f(acc, C::map(row, compute)))
    }
}

impl<S, T, C> ReadOnlyClone for LazyColumnarVec<S, T, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
    T: VecValue,
{
    type ReadOnly = Self;

    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}
