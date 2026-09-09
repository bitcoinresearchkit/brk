use std::{fmt::Debug, iter::once, sync::Arc};

use crate::{Cursor, Error, ReadableVec, Result, VecIndex, VecValue, Version};

use super::{LazyColumnSumVec, LazyColumnVec};

const FNV_OFFSET: u32 = 0x811c_9dc5;
const FNV_PRIME: u32 = 0x0100_0193;

/// Typed description of a fixed column set and its logical row representation.
///
/// Column debug identities in [`Self::ALL`] order are automatically included
/// in the persisted schema version.
pub trait ColumnId: Copy + Debug + Eq + Ord + Send + Sync + 'static {
    type Row<T>: VecValue
    where
        T: VecValue;

    /// Bump this whenever a column's meaning changes without changing its
    /// debug identity or physical ordering.
    const VERSION: Version;

    /// Every valid column in physical storage order.
    const ALL: &'static [Self];

    fn index(self) -> usize;

    fn get<T: VecValue>(self, row: &Self::Row<T>) -> &T;

    fn get_mut<T: VecValue>(self, row: &mut Self::Row<T>) -> &mut T;

    fn from_fn<T, F>(f: F) -> Self::Row<T>
    where
        T: VecValue,
        F: FnMut(Self) -> T;

    fn map<T, U, F>(row: Self::Row<T>, f: F) -> Self::Row<U>
    where
        T: VecValue,
        U: VecValue,
        F: FnMut(T) -> U;

    fn map_ref<T, U, F>(row: &Self::Row<T>, mut f: F) -> Self::Row<U>
    where
        T: VecValue,
        U: VecValue,
        F: FnMut(&T) -> U,
    {
        Self::from_fn(|column| f(column.get(row)))
    }
}

/// Read-only access to a source that preserves typed column boundaries.
pub trait ReadableColumnarVec<C>: ReadableVec<Self::I, C::Row<Self::T>> + Clone
where
    C: ColumnId,
{
    /// A full scalar snapshot. Stored sources reuse their column's existing cache.
    fn column_snapshot(&self, column: C) -> Arc<Vec<Self::T>> {
        Arc::new(
            self.column("", Version::ZERO, column)
                .collect_range_at(0, self.len()),
        )
    }
    type I: VecIndex;
    type T: VecValue;

    /// Visits selected columns in the requested order within row-aligned chunks.
    ///
    /// `row_start` is the absolute index of the first value in `values`.
    fn for_each_column_chunk_at<F>(&self, columns: &[C], from: usize, to: usize, f: &mut F)
    where
        F: FnMut(C, usize, &[Self::T]);

    /// Append a single column's values at sorted row indices, preserving duplicates.
    fn read_column_sorted_into_at(&self, column: C, indices: &[usize], out: &mut Vec<Self::T>) {
        validate_column(column);
        let projection = self.column("sorted_column", Version::ZERO, column);
        let mut cursor = Cursor::new(&projection);
        out.extend(indices.iter().filter_map(|&index| cursor.get(index)));
    }

    /// Visits requested rows of each selected column, in column order.
    /// Implementations with a publication gate keep it across all columns.
    fn for_each_column_sorted_at<F>(&self, columns: &[C], indices: &[usize], f: &mut F)
    where
        F: FnMut(C, &[Self::T]),
    {
        let mut values = Vec::with_capacity(indices.len());
        for &column in columns {
            values.clear();
            self.read_column_sorted_into_at(column, indices, &mut values);
            f(column, &values);
        }
    }

    fn column(&self, name: &str, version: Version, column: C) -> LazyColumnVec<Self, C>
    where
        Self: Sized,
    {
        LazyColumnVec::new(name, version, self.clone(), column)
    }

    fn sum_columns(
        &self,
        name: &str,
        version: Version,
        columns: impl IntoIterator<Item = C>,
    ) -> LazyColumnSumVec<Self, C>
    where
        Self: Sized,
    {
        LazyColumnSumVec::new(name, version, self.clone(), columns)
    }
}

pub fn validate_schema<C: ColumnId>() -> Result<Version> {
    if C::ALL.is_empty() {
        return Err(Error::InvalidArgument(
            "ColumnarVec requires at least one column",
        ));
    }

    let mut fingerprint = FNV_OFFSET;
    let mut names = Vec::with_capacity(C::ALL.len());
    for (index, &column) in C::ALL.iter().enumerate() {
        if column.index() != index {
            return Err(Error::InvalidArgument(
                "ColumnId::ALL must contain every column in physical index order",
            ));
        }
        let name = format!("{column:?}");
        if name.is_empty() {
            return Err(Error::InvalidArgument(
                "ColumnId column names cannot be empty",
            ));
        }
        if names.contains(&name) {
            return Err(Error::InvalidArgument(
                "ColumnId column names must be unique",
            ));
        }
        for byte in name.bytes().chain(once(0)) {
            fingerprint ^= u32::from(byte);
            fingerprint = fingerprint.wrapping_mul(FNV_PRIME);
        }
        names.push(name);
    }
    Ok(Version::new(fingerprint))
}

pub fn validate_column<C: ColumnId>(column: C) {
    let index = column.index();
    assert_eq!(
        C::ALL.get(index),
        Some(&column),
        "invalid column ID at physical index {index}",
    );
}
