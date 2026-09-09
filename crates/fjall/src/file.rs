// Copyright (c) 2024-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

#[cfg(not(target_os = "windows"))]
use log::error as LogError;
#[cfg(not(target_os = "windows"))]
use std::fs::File;
#[cfg(any(not(target_os = "windows"), target_os = "windows"))]
use std::io::Result;
#[cfg(any(not(target_os = "windows"), target_os = "windows"))]
use std::path::Path;

pub const DATABASE_FORMAT: &[u8] = b"FJL\x08";
pub const KEYSPACES_FOLDER: &str = "keyspaces";

pub const LOCK_FILE: &str = "lock";
pub const VERSION_MARKER: &str = "version";

#[cfg(not(target_os = "windows"))]
pub fn fsync_directory(path: &Path) -> Result<()> {
    let file = File::open(path).inspect_err(|error| {
        LogError!("Failed to open directory at {}: {error:?}", path.display());
    })?;
    debug_assert!(file.metadata()?.is_dir());
    file.sync_all().inspect_err(|error| {
        LogError!("Failed to fsync directory at {}: {error:?}", path.display());
    })
}

#[cfg(target_os = "windows")]
pub fn fsync_directory(_path: &Path) -> Result<()> {
    Ok(())
}
