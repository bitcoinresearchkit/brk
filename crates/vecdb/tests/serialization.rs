#![cfg(feature = "serde")]

use vecdb::{
    AnySerializableVec, AnyStoredVec, BytesVec, Database, ImportableVec, MutableVec, StoredVec,
    Version, WritableVec,
};

#[test]
fn json_ranges_follow_emitted_values_including_holes() -> vecdb::Result<()> {
    let directory = tempfile::tempdir()?;
    let db = Database::open(directory.path())?;
    let mut values = MutableVec::<BytesVec<usize, u32>>::import(&db, "values", Version::ONE)?;
    for value in [10, 20, 30, 40] {
        values.push(value);
    }
    values.delete_at(0);
    values.delete_at(2);
    values.write()?;
    let published = values.read_only_clone();

    for source in [&values as &dyn AnySerializableVec, &published] {
        for (from, to, expected) in [
            (None, None, "[20,40]"),
            (Some(0), Some(1), "[]"),
            (Some(2), Some(3), "[]"),
            (Some(0), Some(2), "[20]"),
            (Some(2), Some(4), "[40]"),
            (Some(1), Some(1), "[]"),
            (Some(3), Some(1), "[]"),
            (Some(10), None, "[]"),
        ] {
            let mut output = b"prefix:".to_vec();
            source.write_json(from, to, &mut output)?;
            assert_eq!(output, format!("prefix:{expected}").as_bytes());
        }
    }
    values.delete_at(1);
    values.delete_at(3);
    values.write()?;
    for source in [&values as &dyn AnySerializableVec, &published] {
        let mut output = Vec::new();
        source.write_json(None, None, &mut output)?;
        assert_eq!(output, b"[]");
    }
    Ok(())
}
