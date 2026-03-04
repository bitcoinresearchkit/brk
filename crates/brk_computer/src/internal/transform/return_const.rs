use brk_types::{StoredF32, StoredI8, StoredU16};
use vecdb::UnaryTransform;

/// Returns a constant f32 value from tenths (V=382 -> 38.2), ignoring the input.
pub struct ReturnF32Tenths<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredF32> for ReturnF32Tenths<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredF32 {
        StoredF32::from(V as f32 / 10.0)
    }
}

/// Returns a constant u16 value, ignoring the input.
pub struct ReturnU16<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredU16> for ReturnU16<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredU16 {
        StoredU16::new(V)
    }
}

/// Returns a constant i8 value, ignoring the input.
pub struct ReturnI8<const V: i8>;

impl<S, const V: i8> UnaryTransform<S, StoredI8> for ReturnI8<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredI8 {
        StoredI8::new(V)
    }
}
