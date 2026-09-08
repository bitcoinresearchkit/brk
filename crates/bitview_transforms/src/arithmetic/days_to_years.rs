use brk_types::{StoredF32, StoredF64};
use vecdb::UnaryTransform;

pub struct DaysToYears;

impl UnaryTransform<StoredF32, StoredF32> for DaysToYears {
    #[inline(always)]
    fn apply(value: StoredF32) -> StoredF32 {
        StoredF32::from(*value / 365.0)
    }
}

impl UnaryTransform<StoredF64, StoredF64> for DaysToYears {
    #[inline(always)]
    fn apply(value: StoredF64) -> StoredF64 {
        StoredF64::from(*value / 365.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_trailing_coin_days_to_coin_years() {
        assert_eq!(
            DaysToYears::apply(StoredF64::from(365.0)),
            StoredF64::from(1.0),
        );
        assert_eq!(
            DaysToYears::apply(StoredF64::from(182.5)),
            StoredF64::from(0.5),
        );
        assert!(DaysToYears::apply(StoredF64::NAN).is_nan());
    }
}
