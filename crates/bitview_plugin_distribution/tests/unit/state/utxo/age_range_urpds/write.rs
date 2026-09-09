use std::{collections::BTreeMap, fs::OpenOptions};

use bitview_cohort::{AgeRange, AgeRangeId, UTXOAggregateId};
use brk_types::{CentsCompact, Date, Sats, UrpdRaw};
use tempfile::tempdir;

use super::AgeRangeUrpds;

#[test]
fn packed_file_reads_all_or_one_age_range() {
    let root = tempdir().unwrap();
    let date = Date::new(2026, 8, 23);
    let expected = AgeRangeUrpds {
        entries: AgeRange::from_fn(|id| {
            vec![(
                CentsCompact::new((id.index() as u32 + 1) * 100),
                Sats::from(id.index() as u64 + 1),
            )]
        }),
    };
    expected.write(root.path(), date).unwrap();

    let actual = AgeRangeUrpds::read(root.path(), date).unwrap();
    for id in AgeRangeId::ALL.iter().copied() {
        assert_eq!(actual.get(id), expected.get(id));
    }

    let id = AgeRangeId::From2YTo3Y;
    let one = AgeRangeUrpds::read_one(root.path(), id, date).unwrap();
    assert_eq!(
        one.map,
        expected.get(id).iter().copied().collect::<BTreeMap<_, _>>()
    );

    let mut captured = Vec::new();
    for id in UTXOAggregateId::ALL.iter().copied() {
        assert_eq!(
            AgeRangeUrpds::read_aggregate(root.path(), id, date)
                .unwrap()
                .map,
            expected.aggregate(id).unwrap().map
        );
        let encoded = AgeRangeUrpds::read_aggregate_encoded(root.path(), id, date).unwrap();
        assert_eq!(encoded.sections().count(), id.age_range_ids().len());
        for (section, age) in encoded.sections().zip(id.age_range_ids()) {
            assert_eq!(
                UrpdRaw::deserialize_entries(section).unwrap(),
                expected.get(*age)
            );
        }
        captured.push((id, encoded));
    }

    let bytes = AgeRangeUrpds::read_one_bytes(root.path(), id, date).unwrap();
    assert_eq!(
        UrpdRaw::deserialize_entries(&bytes).unwrap(),
        expected.get(id)
    );
    let replacement = AgeRangeUrpds {
        entries: AgeRange::from_fn(|_| Vec::new()),
    };
    replacement.write(root.path(), date).unwrap();
    for (id, encoded) in captured {
        assert_eq!(
            encoded.decode().unwrap().map,
            expected.aggregate(id).unwrap().map
        );
    }
    assert!(
        AgeRangeUrpds::read_one(root.path(), id, date)
            .unwrap()
            .map
            .is_empty()
    );
    assert_eq!(
        UrpdRaw::deserialize_entries(&bytes).unwrap(),
        expected.get(id)
    );
    OpenOptions::new()
        .write(true)
        .open(AgeRangeUrpds::path(root.path(), date))
        .unwrap()
        .set_len(UrpdRaw::MAX_ENCODED_BYTES as u64 + 1)
        .unwrap();
    assert!(AgeRangeUrpds::read(root.path(), date).is_err());
    assert!(AgeRangeUrpds::read_one_bytes(root.path(), id, date).is_err());
    assert!(
        AgeRangeUrpds::read_aggregate_encoded(root.path(), UTXOAggregateId::All, date).is_err()
    );
    assert!(
        AgeRangeUrpds::read_aggregate_encoded(root.path(), UTXOAggregateId::Sth, date).is_err()
    );
}
