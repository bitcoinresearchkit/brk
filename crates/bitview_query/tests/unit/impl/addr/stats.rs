use super::*;

#[test]
fn balances_reject_unreconciled_spends_and_checked_overflow() {
    let sats = Sats::from;
    assert_eq!(
        address_balances(sats(100u64), sats(40u64), sats(10u64), sats(20u64)).unwrap(),
        (sats(60u64), sats(50u64))
    );
    assert!(matches!(
        address_balances(sats(100u64), sats(100u64), Sats::ZERO, sats(100u64)),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        address_balances(sats(u64::MAX), Sats::ZERO, sats(1u64), Sats::ZERO),
        Err(Error::StateUpdating)
    ));
    assert!(matches!(
        address_balances(sats(1u64), sats(2u64), Sats::ZERO, Sats::ZERO),
        Err(Error::Internal(_))
    ));
}
