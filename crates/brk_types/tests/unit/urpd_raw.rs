use super::*;
use crate::PercentileId;

#[test]
fn malformed_snapshot_returns_errors() {
    let valid =
        UrpdRaw::serialize_iter([(CentsCompact::new(100), Sats::from(1_u64))].into_iter()).unwrap();
    for (offset, value) in [
        (0, 2_usize),
        (0, UrpdRaw::MAX_ENTRIES + 1),
        (0, usize::MAX),
        (8, usize::MAX),
        (16, usize::MAX),
    ] {
        let mut bytes = valid.clone();
        bytes[offset..offset + 8].copy_from_slice(value.to_le_bytes().as_ref());
        assert!(UrpdRaw::deserialize_exact(&bytes).is_err());
    }
    for prices in [[100, 100], [200, 100]] {
        let bytes = UrpdRaw::serialize_iter(
            prices
                .into_iter()
                .map(|price| (CentsCompact::new(price), Sats::from(1_u64))),
        )
        .unwrap();
        assert!(UrpdRaw::deserialize_exact(&bytes).is_err());
    }
    for end in [0, 23, valid.len() - 1] {
        assert!(UrpdRaw::deserialize_exact(&valid[..end]).is_err());
    }
}

#[test]
fn reserved_price_is_rejected_without_constructing_a_nan_price() {
    for (price, supply) in [(u32::MAX, 1_u64), (100, u64::MAX)] {
        let keys = simple_compress(&[price], &ChunkConfig::default()).unwrap();
        let values = simple_compress(&[supply], &ChunkConfig::default()).unwrap();
        let mut bytes = Vec::new();
        for count in [1_usize, keys.len(), values.len()] {
            bytes.extend(count.to_le_bytes());
        }
        bytes.extend(keys);
        bytes.extend(values);
        assert!(UrpdRaw::deserialize_exact(&bytes).is_err());
    }
}

#[test]
fn file_roundtrip() {
    let root = std::env::temp_dir().join(format!("brk-urpd-file-{}", std::process::id()));
    let date = Date::new(2026, 8, 4);
    let expected = BTreeMap::from([
        (CentsCompact::new(100), Sats::from(21_u64)),
        (CentsCompact::new(200), Sats::from(34_u64)),
    ]);

    UrpdRaw::write(
        &root,
        "test",
        date,
        expected.iter().map(|(&price, &sats)| (price, sats)),
    )
    .unwrap();
    let actual = UrpdRaw::read(&root, "test", date).unwrap();

    assert_eq!(actual.map, expected);
    assert_eq!(
        UrpdRaw::read_cost_basis_percentile_prices(&root, "test", date).unwrap(),
        actual.cost_basis_percentile_prices()
    );

    let encoded = UrpdRaw::read_bytes(&root, "test", date).unwrap();
    UrpdRaw::write(
        &root,
        "test",
        date,
        expected.iter().map(|(&price, _)| (price, Sats::ZERO)),
    )
    .unwrap();
    assert_eq!(UrpdRaw::deserialize_exact(&encoded).unwrap().map, expected);
    assert_ne!(UrpdRaw::read(&root, "test", date).unwrap().map, expected);
    let mut trailing = encoded;
    trailing.push(0);
    assert!(UrpdRaw::deserialize_exact(&trailing).is_err());

    UrpdRaw::write(&root, "empty", date, std::iter::empty()).unwrap();
    assert!(UrpdRaw::read(&root, "empty", date).unwrap().map.is_empty());

    let oversized = UrpdRaw::path(&root, "empty", date);
    fs::OpenOptions::new()
        .write(true)
        .open(&oversized)
        .unwrap()
        .set_len(UrpdRaw::MAX_ENCODED_BYTES as u64 + 1)
        .unwrap();
    assert!(UrpdRaw::read_bytes(&root, "empty", date).is_err());

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_supplies_and_non_finite_writer_prices_are_errors() {
    let raw = UrpdRaw {
        map: BTreeMap::from([
            (
                CentsCompact::new(100),
                Sats::from(bitcoin::Amount::MAX_MONEY.to_sat()),
            ),
            (CentsCompact::new(200), Sats::from(1_u64)),
        ]),
    };
    assert!(raw.checked_supply().is_err());
    assert!(raw.serialize().is_err());
    assert!(UrpdRaw::serialize_iter([(CentsCompact::NAN, Sats::ZERO)].into_iter()).is_err());
}

#[test]
fn scalar_weight_floors_each_bucket() {
    let raw = UrpdRaw {
        map: BTreeMap::from([
            (CentsCompact::new(100), Sats::from(3_u64)),
            (CentsCompact::new(200), Sats::from(1_u64)),
        ]),
    };

    assert_eq!(
        raw.apply_weight(0.5).map,
        BTreeMap::from([(CentsCompact::new(100), Sats::from(1_u64))])
    );
}

#[test]
fn cost_basis_percentiles_match_distribution_nearest_rank() {
    let raw = UrpdRaw {
        map: BTreeMap::from([
            (CentsCompact::new(100), Sats::from(5_u64)),
            (CentsCompact::new(200), Sats::from(5_u64)),
        ]),
    };

    let prices = raw.cost_basis_percentile_prices();
    assert_eq!(
        prices.per_coin[PercentileId::Pct50 as usize],
        Cents::new(100)
    );
    assert_eq!(
        prices.per_coin[PercentileId::Pct55 as usize],
        Cents::new(100)
    );
    assert_eq!(
        prices.per_coin[PercentileId::Pct60 as usize],
        Cents::new(200)
    );
    assert_eq!(
        prices.per_dollar[PercentileId::Pct50 as usize],
        Cents::new(200)
    );
}

#[test]
fn empty_cost_basis_percentiles_match_distribution_default() {
    assert_eq!(
        UrpdRaw::default().cost_basis_percentile_prices(),
        CostBasisByPercentile::default()
    );
}
