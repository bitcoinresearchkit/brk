use crate::{
    Format, ReadOnlyCompressedVec, ReadWriteCompressedVec, VecValue, cache::CachePolicy,
    cache::NoCache,
};

pub mod strategy;
pub mod value;

pub use strategy::*;
pub use value::*;

/// Compressed storage using LZ4 for speed-optimized general-purpose compression.
///
/// LZ4 prioritizes compression/decompression speed over ratio, making it ideal
/// for workloads where CPU time matters more than storage space.
///
/// # Performance Characteristics
/// - Extremely fast compression/decompression (hundreds of MB/s)
/// - Moderate compression ratios (typically 2-3x)
/// - Works well with any data type
///
/// # When to Use
/// - Speed is more important than storage savings
/// - Mixed data types (not just numbers)
/// - Need compression but can't afford CPU overhead
#[derive(Debug)]
#[must_use = "Vector should be stored to keep data accessible"]
pub struct LZ4Vec<I, T: VecValue, C: CachePolicy = NoCache>(
    ReadWriteCompressedVec<I, T, LZ4Strategy<T>, C>,
);

impl_vec_wrapper!(
    LZ4Vec,
    ReadWriteCompressedVec<I, T, LZ4Strategy<T>, C>,
    LZ4VecValue,
    Format::LZ4,
    ReadOnlyCompressedVec<I, T, LZ4Strategy<T>, C>
);
