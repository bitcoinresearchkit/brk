#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

mod enums;
mod structs;
mod traits;
mod variants;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
use axum::response::Response;
pub use enums::*;
use memmap2::Mmap;
pub use structs::*;
pub use traits::*;
use variants::*;

#[derive(Debug, Clone)]
pub enum StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    Raw(RawVec<I, T>),
    Compressed(CompressedVec<I, T>),
}

impl<I, T> StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        if *compressed {
            Ok(Self::Compressed(CompressedVec::forced_import(
                path, version,
            )?))
        } else {
            Ok(Self::Raw(RawVec::forced_import(path, version)?))
        }
    }

    pub fn enable_large_cache_if_needed(&mut self) {
        match self {
            StoredVec::Compressed(v) => v.enable_large_cache(),
            Self::Raw(_) => {}
        }
    }
}

impl<I, T> DynamicVec for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type I = I;
    type T = T;

    #[inline]
    fn get_(&self, index: usize) -> Result<Option<Value<T>>> {
        match self {
            StoredVec::Raw(v) => v.get_(index),
            StoredVec::Compressed(v) => v.get_(index),
        }
    }

    #[inline]
    fn stored_len(&self) -> usize {
        match self {
            StoredVec::Raw(v) => v.stored_len(),
            StoredVec::Compressed(v) => v.stored_len(),
        }
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        match self {
            StoredVec::Raw(v) => v.pushed(),
            StoredVec::Compressed(v) => v.pushed(),
        }
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        match self {
            StoredVec::Raw(v) => v.mut_pushed(),
            StoredVec::Compressed(v) => v.mut_pushed(),
        }
    }
}

impl<I, T> GenericVec<I, T> for StoredVec<I, T>
where
    I: StoredIndex + Send + Sync,
    T: StoredType + Send + Sync,
{
    fn iter_from<F>(&mut self, index: I, f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut dyn DynamicVec<I = I, T = T>)) -> Result<()>,
    {
        match self {
            StoredVec::Raw(v) => v.iter_from(index, f),
            StoredVec::Compressed(v) => v.iter_from(index, f),
        }
    }

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<Self::T>> {
        match self {
            StoredVec::Raw(v) => v.collect_range(from, to),
            StoredVec::Compressed(v) => v.collect_range(from, to),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            StoredVec::Raw(v) => v.flush(),
            StoredVec::Compressed(v) => v.flush(),
        }
    }

    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        match self {
            StoredVec::Raw(v) => v.truncate_if_needed(index),
            StoredVec::Compressed(v) => v.truncate_if_needed(index),
        }
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        match self {
            StoredVec::Raw(v) => v.mmap(),
            StoredVec::Compressed(v) => v.mmap(),
        }
    }

    #[inline]
    fn guard(&self) -> &Option<Guard<Arc<Mmap>>> {
        match self {
            StoredVec::Raw(v) => v.guard(),
            StoredVec::Compressed(v) => v.guard(),
        }
    }
    #[inline]
    fn mut_guard(&mut self) -> &mut Option<Guard<Arc<Mmap>>> {
        match self {
            StoredVec::Raw(v) => v.mut_guard(),
            StoredVec::Compressed(v) => v.mut_guard(),
        }
    }

    #[inline]
    fn path(&self) -> &Path {
        match self {
            StoredVec::Raw(v) => v.path(),
            StoredVec::Compressed(v) => v.path(),
        }
    }

    #[inline]
    fn version(&self) -> Version {
        match self {
            StoredVec::Raw(v) => v.version(),
            StoredVec::Compressed(v) => v.version(),
        }
    }
}

pub trait AnyStoredVec: Send + Sync {
    fn file_name(&self) -> String;
    fn index_type_to_string(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn flush(&mut self) -> Result<()>;
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>>;
    fn collect_range_response(&self, from: Option<i64>, to: Option<i64>) -> Result<Response>;
    fn path_vec(&self) -> PathBuf;
}

impl<I, T> AnyStoredVec for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn len(&self) -> usize {
        DynamicVec::len(self)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        DynamicVec::is_empty(self)
    }

    #[inline]
    fn index_type_to_string(&self) -> &str {
        GenericVec::index_type_to_string(self)
    }

    fn flush(&mut self) -> Result<()> {
        GenericVec::flush(self)
    }

    #[inline]
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        GenericVec::collect_range_serde_json(self, from, to)
    }

    #[inline]
    fn collect_range_response(&self, from: Option<i64>, to: Option<i64>) -> Result<Response> {
        GenericVec::collect_range_response(self, from, to)
    }

    #[inline]
    fn path_vec(&self) -> PathBuf {
        GenericVec::path_vec(self)
    }

    fn file_name(&self) -> String {
        GenericVec::file_name(self)
    }
}
