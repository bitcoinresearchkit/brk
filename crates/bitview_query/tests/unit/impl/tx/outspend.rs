use brk_types::{BlockHash, Height, Timestamp};

use super::*;

#[test]
fn invalid_spending_positions_fail_without_panicking() {
    assert!(checked_vin(TxInIndex::from(1usize), TxInIndex::from(2usize)).is_err());
    assert!(checked_vin(TxInIndex::from(65536usize), TxInIndex::from(0usize)).is_err());
    assert_eq!(
        checked_vin(TxInIndex::from(65536usize), TxInIndex::from(1usize)).unwrap(),
        Vin::from(65535usize)
    );
}

#[test]
fn identity_is_content_based_until_a_spending_block_is_known() {
    let bytes = br#"{"spent":false}"#;
    assert!(matches!(
        outspend_identity(&TxOutspend::UNSPENT, bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));

    let unconfirmed = TxOutspend {
        spent: true,
        txid: Some(Txid::COINBASE),
        vin: Some(Vin::from(0usize)),
        status: Some(TxStatus::UNCONFIRMED),
    };
    assert!(matches!(
        outspend_identity(&unconfirmed, bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));

    let hash = BlockHash::default();
    let height = Height::new(42);
    let confirmed = TxOutspend {
        status: Some(TxStatus::confirmed(height, hash, Timestamp::ZERO)),
        ..unconfirmed
    };
    assert!(matches!(
        outspend_identity(&confirmed, bytes),
        RepresentationId::Block(bound_hash) if bound_hash == hash
    ));
}

#[test]
fn array_identity_uses_content_until_every_spend_is_confirmed() {
    let bytes = br#"[{"spent":false}]"#;
    assert!(matches!(
        outspends_identity(&[], bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));
    assert!(matches!(
        outspends_identity(&[TxOutspend::UNSPENT], bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));

    let unconfirmed = TxOutspend {
        spent: true,
        txid: Some(Txid::COINBASE),
        vin: Some(Vin::from(0usize)),
        status: Some(TxStatus::UNCONFIRMED),
    };
    assert!(matches!(
        outspends_identity(&[unconfirmed], bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));

    let confirmed = TxOutspend {
        spent: true,
        txid: Some(Txid::COINBASE),
        vin: Some(Vin::from(0usize)),
        status: Some(TxStatus::confirmed(
            Height::new(10),
            BlockHash::default(),
            Timestamp::ZERO,
        )),
    };
    assert!(matches!(
        outspends_identity(&[confirmed, TxOutspend::UNSPENT], bytes),
        RepresentationId::Content(hash) if hash == RepresentationId::content_hash(bytes)
    ));
}

#[test]
fn array_identity_uses_the_newest_confirmed_spending_block() {
    let older_hash = BlockHash::default();
    let newer_hash =
        BlockHash::try_from("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
    let confirmed = |hash, height| TxOutspend {
        spent: true,
        txid: Some(Txid::COINBASE),
        vin: Some(Vin::from(0usize)),
        status: Some(TxStatus::confirmed(height, hash, Timestamp::ZERO)),
    };
    let outspends = [
        confirmed(newer_hash, Height::new(20)),
        confirmed(older_hash, Height::new(10)),
    ];

    assert!(matches!(
        outspends_identity(&outspends, b"ignored"),
        RepresentationId::Block(hash) if hash == newer_hash
    ));
}
