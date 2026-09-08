use brk_types::{Cents, OHLCCents};
use vecdb::UnaryTransform;

pub struct OhlcCentsToOpenCents;

impl UnaryTransform<OHLCCents, Cents> for OhlcCentsToOpenCents {
    #[inline(always)]
    fn apply(cents: OHLCCents) -> Cents {
        *cents.open
    }
}
