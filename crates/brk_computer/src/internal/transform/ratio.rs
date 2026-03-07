use brk_types::{
    BasisPoints16, BasisPoints32, BasisPointsSigned32, Cents, CentsSigned, Dollars, Sats, StoredF32,
    StoredU32, StoredU64,
};
use vecdb::BinaryTransform;

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

pub struct RatioCentsBp32;

impl BinaryTransform<Cents, Cents, BasisPoints32> for RatioCentsBp32 {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> BasisPoints32 {
        if denominator == Cents::ZERO {
            BasisPoints32::ZERO
        } else {
            BasisPoints32::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}

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

pub struct RatioDollarsBps32;

impl BinaryTransform<Dollars, Dollars, BasisPointsSigned32> for RatioDollarsBps32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPointsSigned32 {
        let ratio = *(numerator / denominator);
        if ratio.is_finite() {
            BasisPointsSigned32::from(ratio)
        } else {
            BasisPointsSigned32::ZERO
        }
    }
}

pub struct RatioCentsSignedCentsBps32;

impl BinaryTransform<CentsSigned, Cents, BasisPointsSigned32> for RatioCentsSignedCentsBps32 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Cents) -> BasisPointsSigned32 {
        if denominator == Cents::ZERO {
            BasisPointsSigned32::ZERO
        } else {
            BasisPointsSigned32::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}

pub struct RatioCentsSignedDollarsBps32;

impl BinaryTransform<CentsSigned, Dollars, BasisPointsSigned32> for RatioCentsSignedDollarsBps32 {
    #[inline(always)]
    fn apply(numerator: CentsSigned, denominator: Dollars) -> BasisPointsSigned32 {
        let d: f64 = denominator.into();
        if d > 0.0 {
            BasisPointsSigned32::from(numerator.inner() as f64 / 100.0 / d)
        } else {
            BasisPointsSigned32::ZERO
        }
    }
}

pub struct RatioDollarsBp32;

impl BinaryTransform<Dollars, Dollars, BasisPoints32> for RatioDollarsBp32 {
    #[inline(always)]
    fn apply(numerator: Dollars, denominator: Dollars) -> BasisPoints32 {
        let ratio = f64::from(numerator) / f64::from(denominator);
        if ratio.is_finite() {
            BasisPoints32::from(ratio)
        } else {
            BasisPoints32::ZERO
        }
    }
}

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
