use brk_types::{BlockHash, Height, Timestamp};

use super::*;

fn spending_positions(
    input_txs: &impl ReadableVec<TxInIndex, TxIndex>,
    first_inputs: &impl ReadableVec<TxIndex, TxInIndex>,
    requested: &mut [(TxInIndex, usize)],
    tx_bound: TxIndex,
) -> Result<Vec<(usize, TxIndex, Vin)>> {
    let ordered = requested.is_sorted_by_key(|&(input, _)| input);
    let mut positions = Vec::with_capacity(requested.len());
    visit_spending_positions(
        input_txs,
        first_inputs,
        requested,
        tx_bound,
        ordered,
        |output, tx, vin| {
            positions.push((output, tx, vin));
            Ok(())
        },
    )?;
    Ok(positions)
}

fn position_fixture(
    db: &vecdb::Database,
    len: usize,
) -> (
    vecdb::PcoVec<TxInIndex, TxIndex>,
    vecdb::PcoVec<TxIndex, TxInIndex>,
) {
    use vecdb::{AnyStoredVec, ImportableVec, Version, WritableVec};
    let mut inputs = vecdb::PcoVec::import(db, "inputs", Version::ONE).unwrap();
    let mut firsts = vecdb::PcoVec::import(db, "firsts", Version::ONE).unwrap();
    for i in 0..len {
        inputs.push(TxIndex::from(i / 3));
    }
    for i in 0..len.div_ceil(3) {
        firsts.push(TxInIndex::from(i * 3));
    }
    inputs.write().unwrap();
    firsts.write().unwrap();
    (inputs, firsts)
}

#[test]
fn batched_spending_positions_keep_output_slots_and_safe_bounds() {
    let dir = tempfile::tempdir().unwrap();
    let db = vecdb::Database::open(dir.path()).unwrap();
    let (inputs, mut firsts) = position_fixture(&db, 32768);
    for requests in [
        vec![],
        vec![12usize],
        vec![32767],
        vec![32766, 0, 21, 22, 21, 5, 17000],
    ] {
        let mut requested: Vec<_> = requests
            .iter()
            .enumerate()
            .map(|(slot, &i)| (TxInIndex::from(i), slot))
            .collect();
        let mut actual =
            spending_positions(&inputs, &firsts, &mut requested, TxIndex::from(10000usize))
                .unwrap();
        actual.sort_unstable_by_key(|&(slot, _, _)| slot);
        let expected: Vec<_> = requests
            .iter()
            .enumerate()
            .filter(|&(_, &i)| i / 3 < 10000)
            .map(|(slot, &i)| (slot, TxIndex::from(i / 3), Vin::from(i % 3)))
            .collect();
        assert_eq!(actual, expected);
    }
    assert!(
        spending_positions(
            &inputs,
            &firsts,
            &mut [
                (TxInIndex::from(0usize), 0),
                (TxInIndex::from(99999usize), 1)
            ],
            TxIndex::from(10000usize)
        )
        .is_err()
    );
    use vecdb::WritableVec;
    firsts.truncate_if_needed_at(1).unwrap();
    assert!(
        spending_positions(
            &inputs,
            &firsts,
            &mut [(TxInIndex::from(6usize), 0), (TxInIndex::from(9usize), 1)],
            TxIndex::from(10000usize)
        )
        .is_err()
    );
}

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
