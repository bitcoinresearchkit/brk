#[cfg(any(unix, not(unix)))]
use crate::Result;

use std::{fmt, fs::File};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Actual disk usage (accounts for sparse files / holes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiskUsage(u64);

impl DiskUsage {
    #[cfg(unix)]
    pub fn from_file(file: &File) -> Result<Self> {
        Ok(Self(file.metadata()?.blocks() * 512))
    }

    #[cfg(not(unix))]
    pub fn from_file(file: &File) -> Result<Self> {
        Ok(Self(file.metadata()?.len()))
    }

    #[inline]
    pub fn bytes(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for DiskUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KIB: u64 = 1024;
        const MIB: u64 = KIB * 1024;
        const GIB: u64 = MIB * 1024;

        let bytes = self.0;
        if bytes >= GIB {
            write!(f, "{:.1} GiB", bytes as f64 / GIB as f64)
        } else if bytes >= MIB {
            write!(f, "{:.1} MiB", bytes as f64 / MIB as f64)
        } else if bytes >= KIB {
            write!(f, "{:.1} KiB", bytes as f64 / KIB as f64)
        } else {
            write!(f, "{} B", bytes)
        }
    }
}
