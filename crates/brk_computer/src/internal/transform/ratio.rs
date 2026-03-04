use brk_types::{
    BasisPoints16, BasisPoints32, BasisPointsSigned16, BasisPointsSigned32, Cents, CentsSigned,
    Dollars, Sats, StoredF32, StoredU32, StoredU64,
};
use vecdb::BinaryTransform;

// === BasisPoints16 (unsigned) ratios ===

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

// === BasisPointsSigned16 (signed) ratios ===

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
            BasisPointsSigned16::from(numerator.inner() as f64 / 100.0 / d)
        } else {
            BasisPointsSigned16::ZERO
        }
    }
}

// === BasisPoints32 (unsigned) ratios ===

/// (Dollars, Dollars) -> BasisPoints32 ratio (a / b × 10000)
pub struct RatioDollarsBp32;

impl BinaryTransform<Dollars, Dollars, BasisPoints32> for RatioDollarsBp32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPoints32 {
        BasisPoints32::from(f64::from(numerator) / f64::from(denominator))
    }
}

// === BasisPointsSigned32 (signed) ratio diffs ===

/// (StoredF32, StoredF32) -> BasisPointsSigned32 ratio diff ((a/b - 1) × 10000)
pub struct RatioDiffF32Bps32;

impl BinaryTransform<StoredF32, StoredF32, BasisPointsSigned32> for RatioDiffF32Bps32 {
    #[inline(always)]
    fn apply(value: StoredF32, base: StoredF32) -> BasisPointsSigned32 {
        if base.is_nan() || *base == 0.0 {
            BasisPointsSigned32::ZERO
        } else {
            BasisPointsSigned32::from((*value / *base - 1.0) as f64)
        }
    }
}

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
