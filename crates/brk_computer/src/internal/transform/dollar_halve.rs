use brk_types::Dollars;
use vecdb::UnaryTransform;

/// Dollars -> Dollars/2 (for supply_half_usd)
pub struct HalveDollars;

impl UnaryTransform<Dollars, Dollars> for HalveDollars {
    #[inline(always)]
    fn apply(dollars: Dollars) -> Dollars {
        dollars.halved()
    }
}
