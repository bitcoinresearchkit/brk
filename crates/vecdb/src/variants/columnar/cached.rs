use std::sync::Arc;

use crate::{
    AnyVec, CachedVec, CachedVecStrategy, Pinned, ReadOnlyClone, ReadableVec, TypedVec, Version,
};

use super::{ColumnId, LazyColumnVec, ReadableColumnarVec, read};

/// Shared scalar-column caches. Sums and projections reuse these caches without
/// caching another row matrix. Admission and eviction belong to each column's
/// cache budget; this is not a chunk cache.
pub struct CachedColumnarVec<S, C, P: CachedVecStrategy = Pinned>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    source: S,
    columns: Arc<[CachedVec<LazyColumnVec<S, C>, P>]>,
}

impl<S, C, P: CachedVecStrategy> Clone for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            columns: self.columns.clone(),
        }
    }
}

impl<S, C, P: CachedVecStrategy> CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    pub fn new(
        source: S,
        version: Version,
        mut wrap: impl FnMut(LazyColumnVec<S, C>) -> CachedVec<LazyColumnVec<S, C>, P>,
    ) -> Self {
        let columns = C::ALL
            .iter()
            .map(|&column| {
                wrap(source.column(
                    &format!("{}_cache_{column:?}", source.name()),
                    version,
                    column,
                ))
            })
            .collect();
        Self { source, columns }
    }

    pub fn invalidate(&self) {
        for column in self.columns.iter() {
            column.invalidate();
        }
    }

    pub fn cached_column(&self, column: C) -> &CachedVec<LazyColumnVec<S, C>, P> {
        super::schema::validate_column(column);
        &self.columns[column.index()]
    }
}

impl<S, C, P: CachedVecStrategy> AnyVec for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn name(&self) -> &str {
        self.source.name()
    }
    fn version(&self) -> Version {
        self.source.version()
    }
    fn len(&self) -> usize {
        self.source.len()
    }
    fn index_type_to_string(&self) -> &'static str {
        self.source.index_type_to_string()
    }
    fn value_type_to_string(&self) -> &'static str {
        self.source.value_type_to_string()
    }
    fn value_type_to_size_of(&self) -> usize {
        self.source.value_type_to_size_of()
    }
    fn region_names(&self) -> Vec<String> {
        self.source.region_names()
    }
}

impl<S, C, P: CachedVecStrategy> TypedVec for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    type I = S::I;
    type T = C::Row<S::T>;
}

impl<S, C, P: CachedVecStrategy> ReadableColumnarVec<C> for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    type I = S::I;
    type T = S::T;

    fn read_column_sorted_into_at(&self, column: C, indices: &[usize], out: &mut Vec<S::T>) {
        self.cached_column(column).read_sorted_into_at(indices, out);
    }

    fn for_each_column_chunk_at<F>(&self, columns: &[C], from: usize, to: usize, f: &mut F)
    where
        F: FnMut(C, usize, &[S::T]),
    {
        read::for_each_column::<S::I, S::T, _, C, _>(
            &self.columns,
            self.len(),
            columns,
            from,
            to,
            f,
        );
    }
}

impl<S, C, P: CachedVecStrategy> ReadableVec<S::I, C::Row<S::T>> for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }
    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<C::Row<S::T>>) {
        read::read_rows::<S::I, S::T, _, C>(&self.columns, self.len(), from, to, out);
    }
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(C::Row<S::T>)) {
        read::fold_readable(self, from, to, (), |(), row| f(row));
    }
    fn fold_range_at<B, F: FnMut(B, C::Row<S::T>) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> B {
        read::fold_readable(self, from, to, init, f)
    }
    fn try_fold_range_at<B, E, F: FnMut(B, C::Row<S::T>) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        read::try_fold_readable(self, from, to, init, f)
    }
}

impl<S, C, P: CachedVecStrategy> ReadOnlyClone for CachedColumnarVec<S, C, P>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    type ReadOnly = Self;
    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}
