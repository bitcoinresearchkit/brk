use brk_types::{BasisPointsSigned32, Cents, Dollars};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> BasisPointsSigned32 ratio diff ((a/b - 1) × 10000)
pub struct RatioDiffDollarsBps32;

impl BinaryTransform<Dollars, Dollars, BasisPointsSigned32> for RatioDiffDollarsBps32 {
    #[inline(always)]
    fn apply(close: Dollars, base: Dollars) -> BasisPointsSigned32 {
        let base_f64: f64 = base.into();
        if base_f64 == 0.0 {
            BasisPointsSigned32::ZERO
        } else {
            BasisPointsSigned32::from(f64::from(close) / base_f64 - 1.0)
        }
    }
}

/// (Cents, Cents) -> BasisPointsSigned32 ratio diff ((a/b - 1) × 10000)
pub struct RatioDiffCentsBps32;

impl BinaryTransform<Cents, Cents, BasisPointsSigned32> for RatioDiffCentsBps32 {
    #[inline(always)]
    fn apply(close: Cents, base: Cents) -> BasisPointsSigned32 {
        let base_f64 = f64::from(base);
        if base_f64 == 0.0 {
            BasisPointsSigned32::ZERO
        } else {
            BasisPointsSigned32::from(f64::from(close) / base_f64 - 1.0)
        }
    }
}
