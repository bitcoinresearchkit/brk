use brk_types::{CheckedSub, Height, Index, PriceRatio, Sats};
use serde_json::{from_str, to_string};

#[cfg(feature = "storage")]
use vecdb::CheckedSub as VecdbCheckedSub;
#[cfg(feature = "storage")]
use vecdb::PrintableIndex;

#[test]
fn domain_operations_and_index_names_do_not_require_storage() {
    assert_eq!(
        CheckedSub::checked_sub(Sats::new(5), Sats::new(2)),
        Some(Sats::new(3))
    );
    assert_eq!(CheckedSub::checked_sub(Sats::new(5), 6_usize), None);
    assert_eq!(
        CheckedSub::checked_sub(Height::new(2), 1_u32),
        Some(Height::new(1))
    );
    assert_eq!(CheckedSub::checked_sub(Height::ZERO, 1_usize), None);
    assert_eq!(
        CheckedSub::checked_sub(PriceRatio::NAN, PriceRatio::ONE),
        Some(PriceRatio::NAN)
    );
    assert_eq!(Height::index_name(), "height");
    for index in Index::all() {
        assert_eq!(Index::try_from(index.name()).unwrap(), index);
        assert!(!index.possible_values().is_empty());
    }
    assert_eq!(to_string(&Sats::new(42)).unwrap(), "42");
    assert_eq!(from_str::<Height>("42").unwrap(), Height::new(42));
}

#[cfg(feature = "storage")]
#[test]
fn storage_adapters_use_the_domain_definitions() {
    for amount in [Sats::ZERO, Sats::new(5), Sats::MAX] {
        for rhs in [0_usize, 5, usize::MAX] {
            assert_eq!(
                VecdbCheckedSub::checked_sub(amount, rhs),
                CheckedSub::checked_sub(amount, rhs),
            );
        }
    }
    assert_eq!(
        <Height as PrintableIndex>::to_string(),
        Height::index_name(),
    );
    assert_eq!(
        <Height as PrintableIndex>::to_possible_strings(),
        Height::index_aliases(),
    );
}
