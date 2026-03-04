use brk_types::{BasisPoints32, Cents, StoredF32};
use vecdb::BinaryTransform;

pub struct PriceTimesRatioCents;

impl BinaryTransform<Cents, StoredF32, Cents> for PriceTimesRatioCents {
    #[inline(always)]
    fn apply(price: Cents, ratio: StoredF32) -> Cents {
        Cents::from(f64::from(price) * f64::from(ratio))
    }
}

pub struct PriceTimesRatioBp32Cents;

impl BinaryTransform<Cents, BasisPoints32, Cents> for PriceTimesRatioBp32Cents {
    #[inline(always)]
    fn apply(price: Cents, ratio: BasisPoints32) -> Cents {
        Cents::from(f64::from(price) * f64::from(ratio))
    }
}
