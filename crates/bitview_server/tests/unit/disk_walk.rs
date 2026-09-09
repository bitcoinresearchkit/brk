use tempfile::tempdir;

use super::{dir_size as walk_dir_size, *};

#[cfg(unix)]
use std::os::unix::fs::symlink;

fn dir_size(path: &Path) -> Result<u64> {
    walk_dir_size(path, &AtomicBool::new(false))
}

#[test]
fn byte_totals_reject_overflow() -> Result<()> {
    assert_eq!(sum_bytes(u64::MAX, 0)?, u64::MAX);
    assert!(sum_bytes(u64::MAX, 1).is_err());
    #[cfg(unix)]
    {
        let largest = u64::MAX / 512;
        assert_eq!(blocks_to_bytes(largest)?, largest * 512);
        assert!(blocks_to_bytes(largest + 1).is_err());
    }
    Ok(())
}

#[test]
fn cancelled_walk_does_not_touch_the_missing_root() {
    let error = walk_dir_size(
        Path::new("missing-disk-fixture-root"),
        &AtomicBool::new(true),
    )
    .unwrap_err();
    assert!(error.to_string().contains("disk scan cancelled"), "{error}");
}

#[test]
fn cancellation_is_checked_before_processing_a_listed_entry() -> Result<()> {
    let directory = tempdir()?;
    fs::write(directory.path().join("data"), [0])?;
    let entry = fs::read_dir(directory.path())?.next().unwrap()?;
    let frame = DirectoryFrame {
        path: directory.path(),
        parent: None,
        depth: 0,
    };
    let cancelled = AtomicBool::new(false);
    cancelled.store(true, Ordering::Relaxed);
    let error = dir_entry_size(entry, &frame, &cancelled).unwrap_err();
    assert!(error.to_string().contains("disk scan cancelled"), "{error}");
    Ok(())
}

#[test]
fn excessive_directory_depth_fails_without_a_partial_total() -> Result<()> {
    let directory = tempdir()?;
    let mut path = directory.path().to_owned();
    for _ in 0..MAX_DEPTH {
        path.push("d");
        fs::create_dir(&path)?;
    }
    assert_eq!(dir_size(directory.path())?, 0);
    fs::create_dir(path.join("too-deep"))?;
    let error = dir_size(directory.path()).unwrap_err();
    assert!(
        error.to_string().contains("depth limit exceeded"),
        "{error}"
    );
    Ok(())
}

#[test]
fn directory_size_counts_nested_allocated_bytes() -> Result<()> {
    let directory = tempdir()?;
    let nested = directory.path().join("nested");
    fs::create_dir(&nested)?;

    let first = directory.path().join("first");
    let second = nested.join("second");
    fs::write(&first, [0; 1])?;
    fs::write(&second, [0; 8192])?;

    let first_bytes = allocated_bytes(&fs::metadata(&first)?)?;
    let second_bytes = allocated_bytes(&fs::metadata(&second)?)?;
    let mut expected = first_bytes + second_bytes;

    #[cfg(unix)]
    {
        symlink(&second, directory.path().join("second-link"))?;
        expected += second_bytes;
    }

    assert_eq!(dir_size(directory.path())?, expected);
    assert_eq!(dir_size(&first)?, first_bytes);
    Ok(())
}

#[cfg(unix)]
#[test]
fn directory_aliases_are_counted_but_cycles_fail() -> Result<()> {
    let directory = tempdir()?;
    let child = directory.path().join("child");
    fs::create_dir(&child)?;
    fs::write(child.join("data"), [0; 8192])?;
    let bytes = dir_size(&child)?;
    symlink(&child, directory.path().join("alias"))?;
    assert_eq!(dir_size(directory.path())?, 2 * bytes);
    symlink(directory.path(), child.join("back"))?;
    let error = dir_size(directory.path()).unwrap_err();
    assert!(
        error.to_string().contains("directory symlink cycle"),
        "{error}"
    );
    Ok(())
}
