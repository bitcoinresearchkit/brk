#![cfg(feature = "pco")]

use tempfile::tempdir;
use vecdb::{AnyStoredVec, Database, ImportableVec, PcoVec, ReadableVec, Version, WritableVec};

#[test]
fn compressed_sorted_gather_handles_raw_tail_pushes_rewrite_and_reopen() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = PcoVec::<usize, u64>::import(&db, "compressed", Version::ONE).unwrap();
    let mut expected: Vec<_> = (0..20_137u64)
        .map(|i| i.wrapping_mul(6364136223846793005))
        .collect();
    for &value in &expected {
        source.push(value);
    }
    source.write().unwrap();
    let stored = expected.len();
    for value in 0..333u64 {
        source.push(value);
        expected.push(value);
    }
    let indices = [
        0,
        1,
        1,
        1023,
        1024,
        1025,
        2047,
        4096,
        19_999,
        20_136,
        20_137,
        20_300,
        20_469,
        20_470,
        usize::MAX,
    ];
    assert_eq!(
        source.read_sorted_at(&indices),
        indices
            .iter()
            .filter_map(|&i| expected.get(i).copied())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        source.read_only_clone().read_sorted_at(&indices),
        indices
            .iter()
            .filter_map(|&i| expected[..stored].get(i).copied())
            .collect::<Vec<_>>()
    );
    source.write().unwrap();
    let mut appended = vec![99];
    source
        .read_only_clone()
        .read_sorted_into_at(&indices, &mut appended);
    assert_eq!(
        &appended[1..],
        indices
            .iter()
            .filter_map(|&i| expected.get(i).copied())
            .collect::<Vec<_>>()
    );
    source.truncate_if_needed_at(10_011).unwrap();
    expected.truncate(10_011);
    for value in 0..2000u64 {
        source.push(value + 7);
        expected.push(value + 7);
    }
    source.write().unwrap();
    drop(source);
    let source = PcoVec::<usize, u64>::import(&db, "compressed", Version::ONE).unwrap();
    let indices: Vec<_> = (0..expected.len())
        .step_by(13)
        .flat_map(|i| [i, i])
        .collect();
    assert_eq!(
        source.read_sorted_at(&indices),
        indices.iter().map(|&i| expected[i]).collect::<Vec<_>>()
    );
    assert!(source.read_sorted_at(&[]).is_empty());
    assert!(source.read_sorted_at(&[usize::MAX]).is_empty());
}
