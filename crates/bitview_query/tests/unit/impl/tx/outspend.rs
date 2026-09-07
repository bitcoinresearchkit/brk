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
#[ignore = "Paired compressed outspend position lookup benchmark"]
fn benchmark_spending_positions() {
    use std::{hint::black_box, time::Instant};
    const N: usize = 262144;
    let dir = tempfile::tempdir().unwrap();
    let db = vecdb::Database::open(dir.path()).unwrap();
    let (inputs, firsts) = position_fixture(&db, N);
    for (name, requested) in [
        ("one", vec![N - 3]),
        ("two", vec![N - 3, N - 2]),
        ("clustered", (N - 32..N).collect()),
        (
            "shuffled32",
            (0..32).map(|i| ((i * 17) % 32) * (N - 1) / 31).collect(),
        ),
        (
            "shuffled4096",
            (0..4096)
                .map(|i| ((i * 7919) % 4096) * (N - 1) / 4095)
                .collect(),
        ),
        ("dense4096", (N - 4096..N).collect()),
    ] {
        let request: Vec<_> = requested
            .iter()
            .enumerate()
            .map(|(slot, &i)| (TxInIndex::from(i), slot))
            .collect();
        let expected: Vec<_> = requested
            .iter()
            .enumerate()
            .map(|(slot, &i)| (slot, TxIndex::from(i / 3), Vin::from(i % 3)))
            .collect();
        let mut times = [Vec::new(), Vec::new()];
        for round in 0..14 {
            for variant in [round % 2, 1 - round % 2] {
                let mut request = request.clone();
                let bound = black_box(TxIndex::from(N));
                let start = Instant::now();
                let mut actual = if variant == 0 {
                    let mut input_cursor = inputs.cursor();
                    let mut first_cursor = firsts.cursor();
                    let mut positions = Vec::with_capacity(request.len());
                    for &(input, slot) in black_box(&request) {
                        let tx = input_cursor.get(input.to_usize()).unwrap();
                        if tx < bound {
                            let first = first_cursor.get(tx.to_usize()).unwrap();
                            positions.push((slot, tx, checked_vin(input, first).unwrap()));
                        }
                    }
                    positions
                } else {
                    spending_positions(&inputs, &firsts, black_box(&mut request), bound).unwrap()
                };
                let elapsed = start.elapsed();
                actual.sort_unstable_by_key(|&(slot, _, _)| slot);
                assert_eq!(actual, expected);
                black_box(actual);
                if round > 1 {
                    times[variant].push(elapsed);
                }
            }
        }
        for times in &mut times {
            times.sort();
        }
        eprintln!(
            "{name}: cursors {:?}, sorted batch {:?}",
            times[0][6], times[1][6]
        );
    }
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
