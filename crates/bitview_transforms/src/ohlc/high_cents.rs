use brk_types::{Cents, OHLCCents};
use vecdb::UnaryTransform;

pub struct OhlcCentsToHighCents;

impl UnaryTransform<OHLCCents, Cents> for OhlcCentsToHighCents {
    #[inline(always)]
    fn apply(cents: OHLCCents) -> Cents {
        *cents.high
    }
}
