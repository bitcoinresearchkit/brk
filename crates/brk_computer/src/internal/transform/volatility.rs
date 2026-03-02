use std::marker::PhantomData;

use brk_types::StoredF32;
use vecdb::UnaryTransform;

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

/// StoredF32 × sqrt(D) -> StoredF32 (annualize daily volatility to D-day period)
pub struct TimesSqrt<D: SqrtDays>(PhantomData<D>);

impl<D: SqrtDays> UnaryTransform<StoredF32, StoredF32> for TimesSqrt<D> {
    #[inline(always)]
    fn apply(v: StoredF32) -> StoredF32 {
        (*v * D::FACTOR).into()
    }
}
