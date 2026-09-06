use std::sync::Arc;

pub mod any_vec;
pub mod read_only_clone;
pub mod readable;
pub mod transform;
pub mod typed;

pub use transform::*;

use crate::{ReadableBoxedVec, VecIndex, VecValue, Version};

/// Lazily computed vector deriving values on-the-fly from one source vector.
///
/// Unlike `EagerVec`, no data is stored on disk. Values are computed during
/// iteration by applying a function to the source vector's elements. Use when:
/// - Storage space is limited
/// - Computation is cheap relative to disk I/O
/// - Values are only accessed once or infrequently
///
/// For frequently accessed derived data, prefer `EagerVec` for better performance.
#[derive(Clone)]
pub struct LazyVec<I, T, S1I, S1T>
where
    S1I: VecIndex,
    S1T: VecValue,
{
    name: Arc<str>,
    base_version: Version,
    source: ReadableBoxedVec<S1I, S1T>,
    compute: fn(I, S1T) -> T,
}

impl<I, T, S1I, S1T> LazyVec<I, T, S1I, S1T>
where
    I: VecIndex,
    T: VecValue,
    S1I: VecIndex,
    S1T: VecValue,
{
    pub fn init(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<S1I, S1T>,
        compute: fn(I, S1T) -> T,
    ) -> Self {
        assert_eq!(
            I::to_string(),
            S1I::to_string(),
            "LazyVec index type mismatch: expected {}, got {}",
            I::to_string(),
            S1I::to_string()
        );

        Self {
            name: Arc::from(name),
            base_version: version,
            source,
            compute,
        }
    }
}

impl<I, T, S1T> LazyVec<I, T, I, S1T>
where
    I: VecIndex,
    T: VecValue,
    S1T: VecValue,
{
    /// Create a lazy vec with a generic transform.
    /// Usage: `LazyVec::transformed::<Negate>(name, v, source)`
    pub fn transformed<F: UnaryTransform<S1T, T>>(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<I, S1T>,
    ) -> Self {
        Self::init(name, version, source, |_, v| F::apply(v))
    }
}
