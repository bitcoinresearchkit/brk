use brk_types::StoredU16;
use vecdb::UnaryTransform;

/// Returns a constant u16 value, ignoring the input.
pub struct ReturnU16<const V: u16>;

impl<S, const V: u16> UnaryTransform<S, StoredU16> for ReturnU16<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredU16 {
        StoredU16::new(V)
    }
}
