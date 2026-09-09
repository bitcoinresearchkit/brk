use std::{fs, path::Path};

use lsm_tree::{Config, Error, Result, Slice, Tree};
use tempfile::tempdir;

fn open(path: &Path) -> Result<Tree> {
    Tree::open(Config::new(path))
}

fn collect(iter: impl Iterator<Item = Result<(Slice, Slice)>>) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    iter.map(|item| {
        let (key, value) = item?;
        Ok((key.to_vec(), value.to_vec()))
    })
    .collect()
}

#[test]
fn ingestion_reads_and_recovery() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;

    let mut ingestion = tree.ingestion()?;
    ingestion.write("a", "1")?;
    ingestion.write("b", "2")?;
    ingestion.write("c", "3")?;
    ingestion.finish()?;

    let mut ingestion = tree.ingestion()?;
    ingestion.write("b", "20")?;
    ingestion.write_weak_tombstone("c")?;
    ingestion.write("d", "4")?;
    ingestion.finish()?;

    assert_eq!(tree.get("a")?.as_deref(), Some(b"1".as_slice()));
    assert_eq!(tree.get("b")?.as_deref(), Some(b"20".as_slice()));
    assert_eq!(tree.get("c")?, None);
    assert_eq!(tree.get("d")?.as_deref(), Some(b"4".as_slice()));
    assert_eq!(
        collect(tree.iter())?,
        [
            (b"a".to_vec(), b"1".to_vec()),
            (b"b".to_vec(), b"20".to_vec()),
            (b"d".to_vec(), b"4".to_vec())
        ],
    );
    assert_eq!(
        collect(tree.range("b"..="d"))?,
        [
            (b"b".to_vec(), b"20".to_vec()),
            (b"d".to_vec(), b"4".to_vec())
        ],
    );

    drop(tree);
    let tree = open(directory.path())?;
    assert_eq!(tree.get("b")?.as_deref(), Some(b"20".as_slice()));
    assert_eq!(tree.get("c")?, None);

    let mut ingestion = tree.ingestion()?;
    ingestion.write("e", "5")?;
    ingestion.finish()?;
    drop(tree);

    assert_eq!(
        open(directory.path())?.get("e")?.as_deref(),
        Some(b"5".as_slice())
    );
    Ok(())
}

#[test]
fn transitive_overlap_preserves_newest_value_after_recovery() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;

    let mut ingestion = tree.ingestion()?;
    ingestion.write("a", "oldest")?;
    ingestion.write("c", "oldest")?;
    ingestion.finish()?;

    let mut ingestion = tree.ingestion()?;
    ingestion.write("a", "middle")?;
    ingestion.write("n", "middle")?;
    ingestion.write("z", "middle")?;
    ingestion.finish()?;

    let mut ingestion = tree.ingestion()?;
    ingestion.write("m", "newest")?;
    ingestion.write("n", "newest")?;
    ingestion.write("p", "newest")?;
    ingestion.finish()?;

    assert_eq!(tree.get("n")?.as_deref(), Some(b"newest".as_slice()));
    drop(tree);
    assert_eq!(
        open(directory.path())?.get("n")?.as_deref(),
        Some(b"newest".as_slice())
    );
    Ok(())
}

#[test]
fn prefix_and_double_ended_ranges() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;
    let mut ingestion = tree.ingestion()?;
    ingestion.write("addr/1", "a")?;
    ingestion.write("addr/2", "b")?;
    ingestion.write("block/1", "c")?;
    ingestion.finish()?;

    assert_eq!(
        collect(tree.prefix("addr/"))?,
        [
            (b"addr/1".to_vec(), b"a".to_vec()),
            (b"addr/2".to_vec(), b"b".to_vec())
        ],
    );

    let mut range = tree.range("addr/1".."block/2");
    assert_eq!(
        range.next().transpose()?.map(|item| item.0),
        Some(Slice::from("addr/1"))
    );
    assert_eq!(
        range.next_back().transpose()?.map(|item| item.0),
        Some(Slice::from("block/1"))
    );
    assert_eq!(
        range.next().transpose()?.map(|item| item.0),
        Some(Slice::from("addr/2"))
    );
    assert!(range.next().is_none());
    Ok(())
}

#[test]
fn compaction_preserves_latest_values_and_open_readers() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;

    for generation in 0..8_u8 {
        let mut ingestion = tree.ingestion()?;
        for key in 0..64_u8 {
            if generation == 7 && key == 10 {
                ingestion.write_weak_tombstone([key])?;
            } else {
                ingestion.write([key], [generation])?;
            }
        }
        ingestion.finish()?;
    }

    let reader = tree.iter();
    tree.compact()?;
    let stable_version = tree.current_version_id();
    tree.compact()?;
    assert_eq!(tree.current_version_id(), stable_version);

    assert_eq!(
        collect(reader)
            .unwrap_or_else(|error| panic!("open reader failed: {error:?}"))
            .len(),
        63,
    );
    for key in 0..64_u8 {
        let expected = (key != 10).then_some([7_u8]);
        assert_eq!(
            tree.get([key])?.as_deref(),
            expected.as_ref().map(|value| value.as_slice())
        );
    }

    drop(tree);
    let tree = open(directory.path())?;
    assert_eq!(collect(tree.iter())?.len(), 63);
    for key in 0..64_u8 {
        let expected = (key != 10).then_some([7_u8]);
        assert_eq!(
            tree.get([key])?.as_deref(),
            expected.as_ref().map(|value| value.as_slice())
        );
    }
    Ok(())
}

#[test]
fn compaction_bounds_overlapping_l0_runs() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;

    for generation in 0..4_u8 {
        let mut ingestion = tree.ingestion()?;
        for key in 0..64_u8 {
            ingestion.write([key], [generation])?;
        }
        ingestion.finish()?;
    }

    assert_eq!(4, tree.l0_run_count());
    let version = tree.current_version_id();
    tree.compact()?;

    assert!(tree.current_version_id() > version);
    assert!(tree.l0_run_count() < 4);
    for key in 0..64_u8 {
        assert_eq!(tree.get([key])?.as_deref(), Some([3_u8].as_slice()));
    }
    Ok(())
}

#[test]
fn empty_ingestion_does_not_publish() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;
    let version = tree.current_version_id();
    tree.ingestion()?.finish()?;
    assert_eq!(tree.current_version_id(), version);
    Ok(())
}

#[test]
fn recovery_rejects_a_truncated_manifest() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;
    let mut ingestion = tree.ingestion()?;
    ingestion.write("a", "1")?;
    ingestion.finish()?;
    drop(tree);

    let path = directory.path().join("current");
    let mut bytes = fs::read(&path)?;
    bytes.truncate(5);
    fs::write(path, bytes)?;

    assert!(matches!(open(directory.path()), Err(Error::Unrecoverable)));
    Ok(())
}

#[test]
fn recovery_upgrades_a_checksumless_v9_manifest() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;
    let mut ingestion = tree.ingestion()?;
    ingestion.write("a", "1")?;
    ingestion.finish()?;
    drop(tree);

    let path = directory.path().join("current");
    let mut bytes = fs::read(&path)?;
    *bytes.get_mut(3).ok_or(Error::Unrecoverable)? = 9;
    bytes.truncate(bytes.len() - size_of::<u128>());
    fs::write(&path, bytes)?;

    let tree = open(directory.path())?;
    assert_eq!(tree.get("a")?.as_deref(), Some(b"1".as_slice()));
    let mut ingestion = tree.ingestion()?;
    ingestion.write("b", "2")?;
    ingestion.finish()?;

    let upgraded = fs::read(path)?;
    assert_eq!(upgraded.get(3), Some(&10));
    Ok(())
}

#[test]
fn recovery_rejects_an_old_manifest_version() -> Result<()> {
    let directory = tempdir()?;
    let tree = open(directory.path())?;
    drop(tree);

    let path = directory.path().join("current");
    let mut bytes = fs::read(&path)?;
    *bytes.get_mut(3).ok_or(Error::Unrecoverable)? = 8;
    fs::write(path, bytes)?;

    assert!(matches!(
        open(directory.path()),
        Err(Error::InvalidVersion(8))
    ));
    Ok(())
}

#[test]
#[should_panic(expected = "ingestion keys must be strictly increasing")]
fn ingestion_rejects_unsorted_keys_in_debug_builds() {
    let directory = tempdir().expect("temporary directory");
    let tree = open(directory.path()).expect("tree");
    let mut ingestion = tree.ingestion().expect("ingestion");
    ingestion.write("b", "1").expect("write");
    ingestion.write("a", "2").expect("write");
}
