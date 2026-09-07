use std::sync::Arc;

use crate::{AnyVec, ReadableVec, TypedVec, Version, short_type_name};

use super::{ColumnId, ReadableColumnarVec, schema::validate_column};

/// Lazy scalar projection of one columnar source.
pub struct LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    name: Arc<str>,
    base_version: Version,
    source: S,
    column: C,
}

impl<S, C> Clone for LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn clone(&self) -> Self {
        Self {
            name: Arc::clone(&self.name),
            base_version: self.base_version,
            source: self.source.clone(),
            column: self.column,
        }
    }
}

impl<S, C> LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    pub fn new(name: &str, version: Version, source: S, column: C) -> Self {
        validate_column(column);
        Self {
            name: name.into(),
            base_version: version,
            source,
            column,
        }
    }

    fn fold_column<B, F>(&self, from: usize, to: usize, init: B, mut f: F) -> B
    where
        F: FnMut(B, S::T) -> B,
    {
        let mut acc = Some(init);
        self.source
            .for_each_column_chunk_at(&[self.column], from, to, &mut |_, _, values| {
                for value in values {
                    acc = Some(f(
                        acc.take().expect("column fold accumulator"),
                        value.clone(),
                    ));
                }
            });
        acc.expect("column fold accumulator")
    }

    fn try_fold_column<B, E, F>(&self, from: usize, to: usize, init: B, mut f: F) -> Result<B, E>
    where
        F: FnMut(B, S::T) -> Result<B, E>,
    {
        let from = from.min(self.source.len());
        let to = to.min(self.source.len());
        let chunk_size = self.source.cursor_chunk_size().max(1);
        let mut acc = Some(init);
        let mut at = from;
        while at < to {
            let end = (at + chunk_size).min(to);
            let mut error = None;
            self.source
                .for_each_column_chunk_at(&[self.column], at, end, &mut |_, _, values| {
                    if error.is_some() {
                        return;
                    }
                    for value in values {
                        let current = acc.take().expect("column fold accumulator");
                        match f(current, value.clone()) {
                            Ok(next) => acc = Some(next),
                            Err(err) => {
                                error = Some(err);
                                break;
                            }
                        }
                    }
                });
            if let Some(error) = error {
                return Err(error);
            }
            at = end;
        }
        Ok(acc.expect("column fold accumulator"))
    }
}

impl<S, C> ReadableVec<S::I, S::T> for LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn cursor_chunk_size(&self) -> usize {
        self.source.cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, out: &mut Vec<S::T>) {
        self.source
            .for_each_column_chunk_at(&[self.column], from, to, &mut |_, _, values| {
                out.extend_from_slice(values)
            });
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(S::T)) {
        self.source
            .for_each_column_chunk_at(&[self.column], from, to, &mut |_, _, values| {
                for value in values {
                    f(value.clone());
                }
            });
    }

    fn fold_range_at<B, F: FnMut(B, S::T) -> B>(&self, from: usize, to: usize, init: B, f: F) -> B {
        self.fold_column(from, to, init, f)
    }

    fn try_fold_range_at<B, E, F: FnMut(B, S::T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        self.try_fold_column(from, to, init, f)
    }
}

impl<S, C> AnyVec for LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    fn version(&self) -> Version {
        self.base_version
            .combine(self.source.version())
            .combine(Version::from(self.column.index() + 1))
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
        size_of::<S::T>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<S::T>()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<S, C> TypedVec for LazyColumnVec<S, C>
where
    C: ColumnId,
    S: ReadableColumnarVec<C>,
{
    type I = S::I;
    type T = S::T;
}
