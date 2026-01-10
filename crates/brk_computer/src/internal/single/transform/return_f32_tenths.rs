use brk_types::StoredF32;
use vecdb::UnaryTransform;

/// Returns a constant f32 value from tenths (V=382 -> 38.2), ignoring the input.
pub struct ReturnF32Tenths<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredF32> for ReturnF32Tenths<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredF32 {
        StoredF32::from(V as f32 / 10.0)
    }
}
