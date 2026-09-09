#![doc = include_str!("../README.md")]
#![allow(clippy::type_complexity)]

use std::mem;

use base::{
    ChangeCursor, ChangeData, ReadOnlyBaseVec, ReadWriteBaseVec, vec_region_name,
    vec_region_name_with,
};
use variants::*;

pub use rawdb::{Database, Error as RawDBError, PAGE_SIZE, Reader, likely, unlikely};

#[cfg(feature = "derive")]
pub use vecdb_derive::{Bytes, Pco};

mod base;
mod bytes;
mod cursor;
mod error;
mod iterators;
mod ops;
mod read_bounds;
mod sparse_read;
mod stamp;
#[macro_use]
mod traits;
mod variants;
mod version;

pub use base::{Format, HEADER_OFFSET, Header, ImportOptions, SharedLen, WithPrev};
pub use bytes::Bytes;

pub use cursor::Cursor;

pub use error::{Error, Result};

pub use iterators::ValueWriter;

pub use ops::{
    BinaryTransform, CheckedSub, Divide, Minus, Plus, ReverseOperands, SaturatingAdd, Times,
};

pub use read_bounds::{BoundedVec, BoundedWriter, ReadBounds};
pub use sparse_read::SparseRead;

pub use stamp::Stamp;

#[cfg(feature = "schemars")]
pub use traits::AnyVecWithSchema;
pub use traits::{
    AnyExportableVec, AnyReadableVec, AnySerializableVec, AnyStoredVec, AnyVec, AnyVecWithWriter,
    Formattable, ImportableVec, PrintableIndex, READ_CHUNK_SIZE, ReadOnlyClone, ReadableBoxedVec,
    ReadableCloneableVec, ReadableOptionVec, ReadableVec, Ro, Rw, StorageMode, StoredVec, TypedVec,
    ValueStrategy, VecIndex, VecValue, WritableVec, i64_to_usize, short_type_name,
};

pub use variants::{
    AggFold, Budgeted, BudgetedCachedVec, BytesStrategy, BytesVec, BytesVecReader, BytesVecValue,
    CacheBudget, CachedBoxedVec, CachedReadableVec, CachedVec, CachedVecBudget, CachedVecStrategy,
    ColumnId, ColumnarVec, CompressedRangeCursor, CompressionStrategy, DeltaAvg, DeltaChange,
    DeltaOp, DeltaRate, DeltaSub, EagerVec, EncodedChunk, Halve, Ident, IndexVec, LazyAggVec,
    LazyColumnSumVec, LazyColumnVec, LazyColumnarVec, LazyDeltaVec, LazyVec, MapOption, MutableVec,
    Negate, NoBudget, OverflowVec, OverflowVecReader, OverflowVecReaderCursor, OverflowVecValue,
    Pinned, PinnedCachedVec, RawRangeCursor, RawStrategy, ReadOnlyColumnarVec,
    ReadOnlyCompressedVec, ReadOnlyMutableVec, ReadOnlyOverflowVec, ReadOnlyRawVec,
    ReadWriteRawVec, ReadableColumnarVec, UnaryTransform, VecReader, VecReaderCursor,
};
#[cfg(feature = "lz4")]
pub use variants::{LZ4Strategy, LZ4Vec, LZ4VecValue};
#[cfg(feature = "pco")]
pub use variants::{Pco, PcoVec, PcoVecValue, PcodecStrategy};
#[cfg(feature = "zerocopy")]
pub use variants::{ZeroCopyStrategy, ZeroCopyVec, ZeroCopyVecValue};
#[cfg(feature = "zstd")]
pub use variants::{ZstdStrategy, ZstdVec, ZstdVecValue};

pub use version::Version;

const ONE_KIB: usize = 1024;

/// Buffer size for reading compressed data (512 KiB).
/// Chosen to balance memory usage with I/O efficiency - large enough to
/// amortize syscall overhead while fitting comfortably in L2/L3 cache.
const BUFFER_SIZE: usize = 512 * ONE_KIB;

const SIZE_OF_U64: usize = mem::size_of::<u64>();
/// Opt-in, calling-thread counters for diagnostic fixtures; absent from default builds.
#[cfg(feature = "diagnostics")]
pub mod diagnostics;
