use std::marker::PhantomData;

use brk_types::{BasisPoints32, Cents, StoredF32, StoredF64, StoredU64, Timestamp};
use vecdb::{BinaryTransform, UnaryTransform};

pub struct PerSec;

impl BinaryTransform<StoredU64, Timestamp, StoredF32> for PerSec {
    #[inline(always)]
    fn apply(count: StoredU64, interval: Timestamp) -> StoredF32 {
        let interval_f64 = f64::from(*interval);
        if interval_f64 > 0.0 {
            StoredF32::from(*count as f64 / interval_f64)
        } else {
            StoredF32::NAN
        }
    }
}

pub struct DaysToYears;

impl UnaryTransform<StoredF32, StoredF32> for DaysToYears {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        StoredF32::from(*v / 365.0)
    }
}

pub trait SqrtDays {
    const FACTOR: f32;
}

pub struct Days7;
impl SqrtDays for Days7 {
    const FACTOR: f32 = 2.6457513; // 7.0_f32.sqrt()
}

pub struct Days30;
impl SqrtDays for Days30 {
    const FACTOR: f32 = 5.477226; // 30.0_f32.sqrt()
}

pub struct Days365;
impl SqrtDays for Days365 {
    const FACTOR: f32 = 19.104973; // 365.0_f32.sqrt()
}

pub struct TimesSqrt<D: SqrtDays>(PhantomData<D>);

impl<D: SqrtDays> UnaryTransform<StoredF32, StoredF32> for TimesSqrt<D> {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        (*v * D::FACTOR).into()
    }
}

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

pub struct RatioCents64;

impl BinaryTransform<Cents, Cents, StoredF64> for RatioCents64 {
    #[inline(always)]
    fn apply(numerator: Cents, denominator: Cents) -> StoredF64 {
        if denominator == Cents::ZERO {
            StoredF64::from(1.0)
        } else {
            StoredF64::from(numerator.inner() as f64 / denominator.inner() as f64)
        }
    }
}
