use std::{hint::black_box, time::Instant};

use tempfile::tempdir;

use super::{dir_size as walk_dir_size, *};

#[cfg(unix)]
use std::os::unix::fs::symlink;

// Preserve the original arithmetic in benchmark-only reference walkers.
fn allocated_bytes(metadata: &Metadata) -> u64 {
    #[cfg(unix)]
    {
        metadata.blocks() * 512
    }
    #[cfg(not(unix))]
    {
        metadata.len()
    }
}

fn dir_size(path: &Path) -> Result<u64> {
    walk_dir_size(path, black_box(&AtomicBool::new(false)))
}

#[test]
#[ignore = "local filesystem traversal comparison"]
fn benchmark_disk_walk() -> Result<()> {
    fn legacy_parallel(path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path)?;
        if metadata.is_dir() {
            legacy_contents(path)
        } else {
            Ok(allocated_bytes(&metadata))
        }
    }

    fn legacy_contents(path: &Path) -> Result<u64> {
        fs::read_dir(path)?
            .par_bridge()
            .map(|entry| {
                let entry = entry?;
                let kind = entry.file_type()?;
                if kind.is_dir() {
                    legacy_contents(&entry.path())
                } else if kind.is_symlink() {
                    legacy_parallel(&entry.path())
                } else {
                    Ok(allocated_bytes(&entry.metadata()?))
                }
            })
            .try_reduce(|| 0, |left, right| Ok(left + right))
    }

    fn sequential(path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path)?;
        if metadata.is_dir() {
            sequential_contents(path)
        } else {
            Ok(allocated_bytes(&metadata))
        }
    }

    fn sequential_contents(path: &Path) -> Result<u64> {
        fs::read_dir(path)?.try_fold(0, |sum, entry| {
            let entry = entry?;
            let kind = entry.file_type()?;
            let bytes = if kind.is_dir() {
                sequential_contents(&entry.path())?
            } else if kind.is_symlink() {
                sequential(&entry.path())?
            } else {
                allocated_bytes(&entry.metadata()?)
            };
            Ok(sum + bytes)
        })
    }

    fn directory_parallel(path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path)?;
        if metadata.is_dir() {
            directory_parallel_contents(path)
        } else {
            Ok(allocated_bytes(&metadata))
        }
    }

    fn directory_parallel_contents(path: &Path) -> Result<u64> {
        let mut files = 0;
        let directories = fs::read_dir(path)?.filter_map(|entry| {
            let next = entry.and_then(|entry| {
                let kind = entry.file_type()?;
                if kind.is_dir() {
                    return Ok(Some(entry.path()));
                }
                let metadata = if kind.is_symlink() {
                    fs::metadata(entry.path())?
                } else {
                    entry.metadata()?
                };
                if metadata.is_dir() {
                    return Ok(Some(entry.path()));
                }
                files += allocated_bytes(&metadata);
                Ok(None)
            });
            match next {
                Ok(Some(path)) => Some(Ok(path)),
                Ok(None) => None,
                Err(error) => Some(Err(error)),
            }
        });
        let children = directories
            .par_bridge()
            .map(|path| directory_parallel_contents(&path?))
            .try_reduce(|| 0, |left, right| Ok(left + right))?;
        Ok(files + children)
    }

    for (shape, directories, files) in [
        ("flat", 1, 2048),
        ("wide", 64, 32),
        ("deep", 128, 16),
        ("links", 32, 32),
    ] {
        #[cfg(not(unix))]
        if shape == "links" {
            continue;
        }
        let directory = tempdir()?;
        let mut path = directory.path().to_owned();
        for index in 0..directories {
            path = if shape == "deep" {
                path.join("d")
            } else {
                directory.path().join(index.to_string())
            };
            fs::create_dir(&path)?;
            for file in 0..files {
                fs::write(path.join(file.to_string()), [0; 1])?;
            }
        }
        #[cfg(unix)]
        if shape == "links" {
            for index in 0..directories {
                symlink(
                    directory.path().join(index.to_string()),
                    directory.path().join(format!("alias-{index}")),
                )?;
            }
        }
        let expected = dir_size(directory.path())?;
        assert_eq!(sequential(directory.path())?, expected);
        let walkers: [fn(&Path) -> Result<u64>; 4] =
            [legacy_parallel, dir_size, sequential, directory_parallel];
        let mut times = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for iteration in 0..24 {
            // Rotate order; discard four warmup samples per implementation.
            for offset in 0..4 {
                let index = (iteration + offset) % 4;
                let started = Instant::now();
                let total = walkers[index](directory.path())?;
                let elapsed = started.elapsed();
                assert_eq!(black_box(total), expected);
                if iteration >= 4 {
                    times[index].push(elapsed);
                }
            }
        }
        for samples in &mut times {
            samples.sort_unstable();
        }
        eprintln!(
            "{shape}: legacy {:?}, checked {:?}, sequential {:?}, directories {:?} (median; 2048 file visits, 20 warm samples)",
            times[0][10], times[1][10], times[2][10], times[3][10]
        );
    }
    Ok(())
}
