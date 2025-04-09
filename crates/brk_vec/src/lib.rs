#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

mod enums;
mod structs;
mod traits;
mod variants;

use std::{path::Path, sync::Arc};

use arc_swap::{ArcSwap, Guard};
use axum::Json;
pub use enums::*;
use memmap2::Mmap;
pub use structs::*;
pub use traits::*;
pub use variants::*;

#[derive(Debug)]
pub enum StoredVec<I, T> {
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
}

impl<I, T> AnyVec<I, T> for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn get_(&mut self, index: usize) -> Result<Option<Value<T>>> {
        match self {
            StoredVec::Raw(v) => v.get_(index),
            StoredVec::Compressed(v) => v.get_(index),
        }
    }

    fn iter_from<F>(&mut self, index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut Self)) -> Result<()>,
    {
        todo!();
        // match self {
        //     StoredVec::Raw(v) => v.iter_from(index, |(i, t, inner)| f((i, t, self))),
        //     StoredVec::Compressed(v) => v.iter_from(index, |(i, t, inner)| f((i, t, self))),
        // }
    }

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Json<Vec<T>>> {
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
