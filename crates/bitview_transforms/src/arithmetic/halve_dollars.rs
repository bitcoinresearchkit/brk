use brk_types::Dollars;
use vecdb::UnaryTransform;

pub struct HalveDollars;

impl UnaryTransform<Dollars, Dollars> for HalveDollars {
    #[inline(always)]
    fn apply(dollars: Dollars) -> Dollars {
        dollars.halved()
    }
}
