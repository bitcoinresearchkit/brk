use super::*;

#[test]
fn zero_stamp_distinguishes_empty_from_genesis() {
    let zero = Stamp::from(0_u64);

    assert_eq!(next_height_from_min_stamp(zero, false), Height::ZERO);
    assert_eq!(next_height_from_min_stamp(zero, true), Height::new(1));
}

#[test]
fn nonzero_stamp_advances_to_next_height() {
    assert_eq!(
        next_height_from_min_stamp(Stamp::from(41_u64), true),
        Height::new(42)
    );
}
