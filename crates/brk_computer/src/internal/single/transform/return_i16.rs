use brk_types::StoredI16;
use vecdb::UnaryTransform;

/// Returns a constant i16 value, ignoring the input.
pub struct ReturnI16<const V: i16>;

impl<S, const V: i16> UnaryTransform<S, StoredI16> for ReturnI16<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredI16 {
        StoredI16::new(V)
    }
}
