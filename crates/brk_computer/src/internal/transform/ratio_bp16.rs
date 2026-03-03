use brk_types::{BasisPoints16, Cents, Dollars, Sats, StoredU32, StoredU64};
use vecdb::BinaryTransform;

/// (StoredU64, StoredU64) -> BasisPoints16 ratio (a/b × 10000)
pub struct RatioU64Bp16;

impl BinaryTransform<StoredU64, StoredU64, BasisPoints16> for RatioU64Bp16 {
    #[inline(always)]
    fn apply(numerator: StoredU64, denominator: StoredU64) -> BasisPoints16 {
        if *denominator > 0 {
            BasisPoints16::from(*numerator as f64 / *denominator as f64)
        } else {
            BasisPoints16::ZERO
        }
    }
}

/// (Sats, Sats) -> BasisPoints16 ratio (a/b × 10000)
pub struct RatioSatsBp16;

impl BinaryTransform<Sats, Sats, BasisPoints16> for RatioSatsBp16 {
    #[inline(always)]
    fn apply(numerator: Sats, denominator: Sats) -> BasisPoints16 {
        if *denominator > 0 {
            BasisPoints16::from(*numerator as f64 / *denominator as f64)
        } else {
            BasisPoints16::ZERO
        }
    }
}

/// (Cents, Cents) -> BasisPoints16 ratio (a/b × 10000)
pub struct RatioCentsBp16;

impl BinaryTransform<Cents, Cents, BasisPoints16> for RatioCentsBp16 {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> BasisPoints16 {
        if denominator == Cents::ZERO {
            BasisPoints16::ZERO
        } else {
            BasisPoints16::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}

/// (StoredU32, StoredU32) -> BasisPoints16 ratio (a/b × 10000)
pub struct RatioU32Bp16;

impl BinaryTransform<StoredU32, StoredU32, BasisPoints16> for RatioU32Bp16 {
    #[inline(always)]
    fn apply(numerator: StoredU32, denominator: StoredU32) -> BasisPoints16 {
        if *denominator > 0 {
            BasisPoints16::from(*numerator as f64 / *denominator as f64)
        } else {
            BasisPoints16::ZERO
        }
    }
}

/// (Dollars, Dollars) -> BasisPoints16 ratio (a/b × 10000)
pub struct RatioDollarsBp16;

impl BinaryTransform<Dollars, Dollars, BasisPoints16> for RatioDollarsBp16 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPoints16 {
        let ratio = *(numerator / denominator);
        if ratio.is_finite() {
            BasisPoints16::from(ratio)
        } else {
            BasisPoints16::ZERO
        }
    }
}
