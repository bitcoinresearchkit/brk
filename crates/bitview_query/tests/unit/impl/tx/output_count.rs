use super::*;

#[test]
fn stored_output_ranges_are_bounded_before_allocation() {
    for (first, next, published, expected) in [
        (0, 0, 0, 0),
        (7, 10, 12, 3),
        (7, 65_543, 65_543, 65_536),
        (u64::MAX - 1, u64::MAX, u64::MAX, 1),
    ] {
        assert_eq!(
            output_count(
                TxOutIndex::new(first),
                TxOutIndex::new(next),
                TxOutIndex::new(published)
            )
            .unwrap(),
            expected
        );
    }
    for (first, next, published) in [
        (10, 7, 12),
        (7, 10, 9),
        (7, 65_544, 65_544),
        (0, u64::MAX, u64::MAX),
    ] {
        assert!(matches!(
            output_count(
                TxOutIndex::new(first),
                TxOutIndex::new(next),
                TxOutIndex::new(published)
            ),
            Err(Error::Internal(_))
        ));
    }
}
