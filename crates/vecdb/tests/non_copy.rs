#![cfg(feature = "lz4")]

use tempfile::TempDir;
use vecdb::{
    AnyStoredVec, Bytes, Database, Error, ImportableVec, LZ4Vec, ReadableVec, Result, Version,
    WritableVec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapValue(Box<u64>);

impl Bytes for HeapValue {
    type Array = [u8; 8];

    fn to_bytes(&self) -> Self::Array {
        self.0.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 8] = bytes.try_into().map_err(|_| Error::WrongLength {
            expected: 8,
            received: bytes.len(),
        })?;
        Ok(Self(Box::new(u64::from_le_bytes(bytes))))
    }
}

#[test]
fn compressed_fold_clones_non_copy_values() -> Result<()> {
    let temp = TempDir::new()?;
    let db = Database::open(temp.path())?;
    let mut vec: LZ4Vec<usize, HeapValue> = LZ4Vec::import(&db, "heap", Version::ONE)?;

    for value in 0..5_000 {
        vec.push(HeapValue(Box::new(value)));
    }
    vec.write()?;

    let sum = vec.fold(0_u64, |sum, value| sum + *value.0);
    assert_eq!(sum, (0..5_000_u64).sum::<u64>());

    Ok(())
}
