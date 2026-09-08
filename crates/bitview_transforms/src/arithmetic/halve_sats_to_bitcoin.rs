use brk_types::{Bitcoin, Sats};
use vecdb::UnaryTransform;

pub struct HalveSatsToBitcoin;

impl UnaryTransform<Sats, Bitcoin> for HalveSatsToBitcoin {
    #[inline(always)]
    fn apply(sats: Sats) -> Bitcoin {
        Bitcoin::from(sats / 2)
    }
}
