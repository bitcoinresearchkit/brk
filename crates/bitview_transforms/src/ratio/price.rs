use brk_types::{Cents, PriceRatio};
use vecdb::unlikely;

#[inline]
pub fn price_ratio(close: Cents, price: Cents) -> PriceRatio {
    if unlikely(price == Cents::ZERO) {
        PriceRatio::NAN
    } else {
        PriceRatio::from(f64::from(close) / f64::from(price))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_price_has_no_ratio() {
        assert!(price_ratio(Cents::new(100), Cents::ZERO).is_nan());
        assert_eq!(
            price_ratio(Cents::new(100), Cents::new(50)),
            PriceRatio::from(2.0),
        );
        assert_eq!(
            price_ratio(Cents::new(1_000_000), Cents::new(1)),
            PriceRatio::MAX
        );
        assert!(price_ratio(Cents::NAN, Cents::new(1)).is_nan());
        assert!(price_ratio(Cents::new(1), Cents::NAN).is_nan());
        assert_eq!(price_ratio(Cents::ZERO, Cents::new(1)), PriceRatio::ZERO);
    }
}
