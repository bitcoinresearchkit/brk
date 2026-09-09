use std::{marker::PhantomData, sync::Arc};

use parking_lot::RwLock;

use super::{
    ColumnId, ColumnarVec, ReadableColumnarVec,
    read::{fold_readable, for_each_column, read_rows, try_fold_readable},
    schema::validate_column,
};
use crate::{
    AnyVec, BudgetedCachedVec, ReadableVec, SharedLen, StoredVec, TypedVec, Version,
    short_type_name,
};

#[cfg(feature = "diagnostics")]
use crate::diagnostics;

impl<V: StoredVec, C: ColumnId> ColumnarVec<V, C> {
    pub fn read_only_clone(&self) -> ReadOnlyColumnarVec<V, C> {
        ReadOnlyColumnarVec {
            name: Arc::clone(&self.name),
            columns: Arc::clone(&self.read_only_columns),
            visible_rows: self.visible_rows.clone(),
            gate: Arc::clone(&self.gate),
            column_ids: PhantomData,
        }
    }
}

/// Lean read-only clone of a columnar vector.
pub struct ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    name: Arc<str>,
    columns: Arc<[BudgetedCachedVec<V::ReadOnly>]>,
    visible_rows: SharedLen,
    gate: Arc<RwLock<()>>,
    column_ids: PhantomData<C>,
}

impl<V, C> Clone for ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    fn clone(&self) -> Self {
        Self {
            name: Arc::clone(&self.name),
            columns: Arc::clone(&self.columns),
            visible_rows: self.visible_rows.clone(),
            gate: Arc::clone(&self.gate),
            column_ids: PhantomData,
        }
    }
}

impl<V, C> ReadableColumnarVec<C> for ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    type I = V::I;
    type T = V::T;

    fn column_snapshot(&self, column: C) -> Arc<Vec<V::T>> {
        validate_column(column);
        let _guard = self.gate.read();
        let snapshot = self.columns[column.index()].snapshot();
        let visible = self.visible_rows.get();
        if snapshot.len() > visible {
            Arc::new(snapshot[..visible].to_vec())
        } else {
            snapshot
        }
    }

    fn read_column_sorted_into_at(&self, column: C, indices: &[usize], out: &mut Vec<V::T>) {
        validate_column(column);
        let _guard = self.gate.read();
        let len = self.visible_rows.get();
        let indices = &indices[..indices.partition_point(|&i| i < len)];
        #[cfg(feature = "diagnostics")]
        diagnostics::column();
        self.columns[column.index()].read_sorted_into_at(indices, out);
    }

    fn for_each_column_sorted_at<F>(&self, columns: &[C], indices: &[usize], f: &mut F)
    where
        F: FnMut(C, &[V::T]),
    {
        for &column in columns {
            validate_column(column);
        }
        let _guard = self.gate.read();
        let len = self.visible_rows.get();
        let indices = &indices[..indices.partition_point(|&i| i < len)];
        let mut values = Vec::with_capacity(indices.len());
        for &column in columns {
            values.clear();
            #[cfg(feature = "diagnostics")]
            diagnostics::column();
            self.columns[column.index()].read_sorted_into_at(indices, &mut values);
            f(column, &values);
        }
    }

    fn for_each_column_chunk_at<F>(&self, columns: &[C], from: usize, to: usize, f: &mut F)
    where
        F: FnMut(C, usize, &[V::T]),
    {
        for &column in columns {
            validate_column(column);
        }
        let _guard = self.gate.read();
        for_each_column::<V::I, V::T, _, C, F>(
            &self.columns,
            self.visible_rows.get(),
            columns,
            from,
            to,
            f,
        );
    }
}

impl<V, C> ReadableVec<V::I, C::Row<V::T>> for ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    fn cursor_chunk_size(&self) -> usize {
        self.columns[0].cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<C::Row<V::T>>) {
        let _guard = self.gate.read();
        read_rows::<V::I, V::T, _, C>(&self.columns, self.visible_rows.get(), from, to, out);
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(C::Row<V::T>)) {
        fold_readable(self, from, to, (), |(), value| f(value));
    }

    fn fold_range_at<B, F: FnMut(B, C::Row<V::T>) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> B {
        fold_readable(self, from, to, init, f)
    }

    fn try_fold_range_at<B, E, F: FnMut(B, C::Row<V::T>) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        try_fold_readable(self, from, to, init, f)
    }
}

impl<V, C> AnyVec for ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    fn version(&self) -> Version {
        self.columns[0].version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.visible_rows.get()
    }

    fn index_type_to_string(&self) -> &'static str {
        self.columns[0].index_type_to_string()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<C::Row<V::T>>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<C::Row<V::T>>()
    }

    fn region_names(&self) -> Vec<String> {
        self.columns.iter().flat_map(AnyVec::region_names).collect()
    }
}

impl<V, C> TypedVec for ReadOnlyColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    type I = V::I;
    type T = C::Row<V::T>;
}
