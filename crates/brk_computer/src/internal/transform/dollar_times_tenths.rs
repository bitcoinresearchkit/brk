use brk_types::Dollars;
use vecdb::UnaryTransform;

/// Dollars * (V/10) -> Dollars (e.g., V=8 -> * 0.8, V=24 -> * 2.4)
pub struct DollarsTimesTenths<const V: u16>;

impl<const V: u16> UnaryTransform<Dollars, Dollars> for DollarsTimesTenths<V> {
    #[inline(always)]
    fn apply(d: Dollars) -> Dollars {
        d * (V as f64 / 10.0)
    }
}
