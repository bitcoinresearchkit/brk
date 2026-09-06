use std::marker::PhantomData;

pub mod raw;
pub mod value;

/// Serialization strategy using the Bytes trait with portable byte order.
///
/// Implements little-endian serialization for cross-platform compatibility.
#[derive(Debug, Clone, Copy)]
pub struct BytesStrategy<T>(PhantomData<T>);
