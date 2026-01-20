use brk_types::StoredI8;
use vecdb::UnaryTransform;

/// Returns a constant i8 value, ignoring the input.
pub struct ReturnI8<const V: i8>;

impl<S, const V: i8> UnaryTransform<S, StoredI8> for ReturnI8<V> {
    #[inline(always)]
    fn apply(_: S) -> StoredI8 {
        StoredI8::new(V)
    }
}
