use brk_types::{BasisPointsSigned16, Cents, CentsSigned, Dollars};
use vecdb::BinaryTransform;

/// (Dollars, Dollars) -> BasisPointsSigned16 ratio (a/b × 10000)
pub struct RatioDollarsBps16;

impl BinaryTransform<Dollars, Dollars, BasisPointsSigned16> for RatioDollarsBps16 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPointsSigned16 {
        let ratio = *(numerator / denominator);
        if ratio.is_finite() {
            BasisPointsSigned16::from(ratio)
        } else {
            BasisPointsSigned16::ZERO
        }
    }
}

/// (Dollars, Dollars) -> BasisPointsSigned16 negated ratio (-(a/b) × 10000)
pub struct NegRatioDollarsBps16;

impl BinaryTransform<Dollars, Dollars, BasisPointsSigned16> for NegRatioDollarsBps16 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPointsSigned16 {
        let ratio = *(numerator / denominator);
        if ratio.is_finite() {
            BasisPointsSigned16::from(-ratio)
        } else {
            BasisPointsSigned16::ZERO
        }
    }
}

/// (CentsSigned, Cents) -> BasisPointsSigned16 ratio (a/b × 10000)
pub struct RatioCentsSignedCentsBps16;

impl BinaryTransform<CentsSigned, Cents, BasisPointsSigned16> for RatioCentsSignedCentsBps16 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Cents) -> BasisPointsSigned16 {
        if denominator == Cents::ZERO {
            BasisPointsSigned16::ZERO
        } else {
            BasisPointsSigned16::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}

/// (CentsSigned, Dollars) -> BasisPointsSigned16 ratio (a/b × 10000)
pub struct RatioCentsSignedDollarsBps16;

impl BinaryTransform<CentsSigned, Dollars, BasisPointsSigned16> for RatioCentsSignedDollarsBps16 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Dollars) -> BasisPointsSigned16 {
        let d: f64 = denominator.into();
        if d > 0.0 {
            // Convert cents to dollars first, then compute ratio
            BasisPointsSigned16::from(numerator.inner() as f64 / 100.0 / d)
        } else {
            BasisPointsSigned16::ZERO
        }
    }
}
