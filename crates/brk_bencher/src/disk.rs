use std::collections::HashMap;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct DiskMonitor {
    cache: HashMap<PathBuf, (u64, SystemTime)>, // path -> (bytes_used, mtime)
}

impl DiskMonitor {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get disk usage in bytes (matches `du` and Finder)
    pub fn get_disk_usage(&mut self, path: &Path) -> io::Result<u64> {
        self.scan_recursive(path)
    }

    fn scan_recursive(&mut self, path: &Path) -> io::Result<u64> {
        let mut total = 0;

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                let mtime = metadata.modified()?;

                // Check cache: if mtime unchanged, use cached value
                if let Some((cached_bytes, cached_mtime)) = self.cache.get(&path)
                    && *cached_mtime == mtime
                {
                    total += cached_bytes;
                    continue;
                }

                // File is new or modified - get actual disk usage
                let bytes = metadata.blocks() * 512;
                self.cache.insert(path, (bytes, mtime));
                total += bytes;
            } else if metadata.is_dir() {
                total += self.scan_recursive(&path)?;
            }
        }

        Ok(total)
    }
}
