use super::*;
use crate::{Config, HistogramRaw, Oracle, cents_to_bin};

#[test]
fn zero_historical_price_can_advance_the_histogram() {
    for config in [Config::slow(), Config::default()] {
        let mut oracle = Oracle::new(cents_to_bin(0.0), config);
        oracle.process_histogram(&HistogramRaw::zeros());
        assert_eq!(oracle.price_cents().inner(), 0);
    }
}

#[test]
fn search_and_shape_bounds_do_not_overflow() {
    for bin in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        assert_eq!(search_range(bin, 10, 10), None);
    }
    assert_eq!(search_range(f64::MAX, 10, 10), None);
    assert_eq!(
        search_range(10.0, usize::MAX, usize::MAX),
        Some(0..NUM_BINS)
    );
    assert_eq!(search_range(100.0, 10, 20), Some(90..121));
    for center in [i64::MIN, i64::MAX] {
        assert!(
            arms_at(&HistogramEma::zeros(), center)
                .iter()
                .all(|v| *v == 0.0)
        );
    }
}
