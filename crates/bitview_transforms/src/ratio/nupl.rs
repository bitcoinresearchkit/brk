use brk_types::{PartsPerMillionSigned32, PriceRatio};
use vecdb::UnaryTransform;

pub struct MvrvToNupl;

impl UnaryTransform<PriceRatio, PartsPerMillionSigned32> for MvrvToNupl {
    #[inline(always)]
    fn apply(mvrv: PriceRatio) -> PartsPerMillionSigned32 {
        PartsPerMillionSigned32::from(1.0 - 1.0 / f64::from(mvrv))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nupl_is_derived_from_mvrv() {
        assert_eq!(
            MvrvToNupl::apply(PriceRatio::from(2.0)),
            PartsPerMillionSigned32::from(0.5),
        );
        assert_eq!(
            MvrvToNupl::apply(PriceRatio::from(1.0)),
            PartsPerMillionSigned32::ZERO,
        );
        assert!(MvrvToNupl::apply(PriceRatio::NAN).is_nan());
        assert!(MvrvToNupl::apply(PriceRatio::ZERO).is_nan());
        assert_eq!(
            MvrvToNupl::apply(PriceRatio::from(10_000.0)),
            PartsPerMillionSigned32::from(0.999_767),
        );
    }
}
