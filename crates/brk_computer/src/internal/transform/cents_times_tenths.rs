use brk_types::Cents;
use vecdb::UnaryTransform;

/// Cents * (V/10) -> Cents (e.g., V=8 -> * 0.8, V=24 -> * 2.4)
pub struct CentsTimesTenths<const V: u16>;

impl<const V: u16> UnaryTransform<Cents, Cents> for CentsTimesTenths<V> {
    #[inline(always)]
    fn apply(c: Cents) -> Cents {
        // Use u128 to avoid overflow: c * V / 10
        Cents::from(c.as_u128() * V as u128 / 10)
    }
}
