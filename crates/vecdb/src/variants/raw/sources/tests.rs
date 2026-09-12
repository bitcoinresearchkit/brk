use crate::{
    AnyStoredVec, Budgeted, Bytes, BytesVec, Database, Error, ImportableVec, RawIoSource,
    RawMmapSource, ReadableVec, Result, VecValue, Version, WritableVec,
};
use tempfile::tempdir;

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapValue(Box<u64>);

impl Bytes for HeapValue {
    type Array = [u8; 8];

    fn to_bytes(&self) -> Self::Array {
        self.0.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes = bytes.try_into().map_err(|_| Error::WrongLength {
            expected: 8,
            received: bytes.len(),
        })?;
        Ok(Self(Box::new(u64::from_le_bytes(bytes))))
    }
}

fn check_bulk_reads<T: Bytes + VecValue + Eq>(values: Vec<T>) {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        Budgeted::init_global(1024 * 1024).unwrap();
    });
    let budget = Budgeted::global().unwrap();
    let mut cached = BytesVec::<usize, T, Budgeted>::import(&db, "cached", Version::ONE).unwrap();
    let mut plain = BytesVec::<usize, T>::import(&db, "plain", Version::ONE).unwrap();
    for value in &values {
        cached.push(value.clone());
        plain.push(value.clone());
    }
    cached.write().unwrap();
    plain.write().unwrap();
    let cached_reader = cached.read_only_clone();
    let plain_reader = plain.read_only_clone();
    for (from, to) in [(117, 9731), (9000, usize::MAX), (42, 42), (9999, 9000)] {
        let end = to.min(values.len());
        let mut expected = vec![values[0].clone()];
        if from < end {
            expected.extend_from_slice(&values[from..end]);
        }
        for source in [
            &cached as &dyn ReadableVec<usize, T>,
            &cached_reader,
            &plain,
            &plain_reader,
        ] {
            budget.clear();
            for _ in 0..2 {
                let mut out = vec![values[0].clone()];
                source.read_into_at(from, to, &mut out);
                assert_eq!(out, expected);
            }
        }
        let mut out = vec![values[0].clone()];
        RawMmapSource::new(&plain, from, to).read_into(&mut out);
        assert_eq!(out, expected);
        // Force the I/O path even when the file's pages are resident.
        let mut out = vec![values[0].clone()];
        RawIoSource::new(&plain, from, to).read_into(&mut out);
        assert_eq!(out, expected);
    }
}

#[test]
fn native_bulk_reads_match_io_mmap_and_cached_views() {
    check_bulk_reads((0..10_000u64).map(|value| value * 37).collect());
}

#[test]
fn non_native_bulk_reads_decode_owned_values() {
    check_bulk_reads(
        (0..10_000u64)
            .map(|value| HeapValue(Box::new(value * 37)))
            .collect(),
    );
}
use std::sync::Once;
