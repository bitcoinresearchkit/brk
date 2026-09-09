use crate::{AnyVec, ReadableVec, StoredVec, VecIndex, VecValue};

use super::{ColumnId, ColumnarVec};

pub fn read_rows<I, T, R, C>(
    columns: &[R],
    rows: usize,
    from: usize,
    to: usize,
    out: &mut Vec<C::Row<T>>,
) where
    I: VecIndex,
    T: VecValue,
    R: ReadableVec<I, T>,
    C: ColumnId,
{
    let from = from.min(rows);
    let to = to.min(rows);
    if from >= to {
        return;
    }

    debug_assert_eq!(columns.len(), C::ALL.len());
    out.reserve(to - from);
    let chunk_size = columns[0].cursor_chunk_size().max(1);
    let mut values = C::from_fn(|_| Vec::with_capacity(chunk_size.min(to - from)));
    let mut at = from;
    while at < to {
        let end = (at + chunk_size).min(to);
        let take = end - at;
        for &column in C::ALL {
            let column_values = column.get_mut(&mut values);
            column_values.clear();
            columns[column.index()].read_into_at(at, end, column_values);
        }
        assert!(
            C::ALL
                .iter()
                .all(|&column| column.get(&values).len() == take),
            "column read returned incomplete rows",
        );
        out.extend((0..take).map(|index| C::from_fn(|column| column.get(&values)[index].clone())));
        at = end;
    }
}

pub fn for_each_column<I, T, R, C, F>(
    sources: &[R],
    rows: usize,
    columns: &[C],
    from: usize,
    to: usize,
    f: &mut F,
) where
    I: VecIndex,
    T: VecValue,
    R: ReadableVec<I, T>,
    C: ColumnId,
    F: FnMut(C, usize, &[T]),
{
    let from = from.min(rows);
    let to = to.min(rows);
    if from >= to {
        return;
    }

    for &column in columns {
        let mut emitted = 0;
        sources[column.index()].for_each_chunk_at(from, to, &mut |at, values| {
            emitted += values.len();
            f(column, at, values);
        });
        assert_eq!(emitted, to - from, "column read returned incomplete rows");
    }
}

impl<V, C> ReadableVec<V::I, C::Row<V::T>> for ColumnarVec<V, C>
where
    V: StoredVec,
    C: ColumnId,
{
    #[inline(always)]
    fn collect_one_at(&self, index: usize) -> Option<C::Row<V::T>> {
        let stored_rows = self.stored_rows();
        if index >= stored_rows {
            return self.pushed.get(index - stored_rows).cloned();
        }

        let mut row = Vec::with_capacity(1);
        read_rows::<V::I, V::T, _, C>(&self.columns, stored_rows, index, index + 1, &mut row);
        row.pop()
    }

    fn cursor_chunk_size(&self) -> usize {
        self.first().cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<C::Row<V::T>>) {
        let len = self.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return;
        }

        let stored_rows = self.stored_rows();
        if from < stored_rows {
            read_rows::<V::I, V::T, _, C>(
                &self.columns,
                stored_rows,
                from,
                to.min(stored_rows),
                out,
            );
        }
        if to > stored_rows {
            let start = from.max(stored_rows) - stored_rows;
            let end = to - stored_rows;
            out.extend_from_slice(&self.pushed[start..end]);
        }
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

pub fn fold_readable<I, T, R, B, F>(vec: &R, from: usize, to: usize, mut acc: B, mut f: F) -> B
where
    I: VecIndex,
    T: VecValue,
    R: ReadableVec<I, T>,
    F: FnMut(B, T) -> B,
{
    let from = from.min(vec.len());
    let to = to.min(vec.len());
    let chunk = vec.cursor_chunk_size().max(1);
    let mut buf = Vec::with_capacity(chunk.min(to.saturating_sub(from)));
    let mut at = from;
    while at < to {
        let end = (at + chunk).min(to);
        buf.clear();
        vec.read_into_at(at, end, &mut buf);
        for value in buf.drain(..) {
            acc = f(acc, value);
        }
        at = end;
    }
    acc
}

pub fn try_fold_readable<I, T, R, B, E, F>(
    vec: &R,
    from: usize,
    to: usize,
    mut acc: B,
    mut f: F,
) -> Result<B, E>
where
    I: VecIndex,
    T: VecValue,
    R: ReadableVec<I, T>,
    F: FnMut(B, T) -> Result<B, E>,
{
    let from = from.min(vec.len());
    let to = to.min(vec.len());
    let chunk = vec.cursor_chunk_size().max(1);
    let mut buf = Vec::with_capacity(chunk.min(to.saturating_sub(from)));
    let mut at = from;
    while at < to {
        let end = (at + chunk).min(to);
        buf.clear();
        vec.read_into_at(at, end, &mut buf);
        for value in buf.drain(..) {
            acc = f(acc, value)?;
        }
        at = end;
    }
    Ok(acc)
}
