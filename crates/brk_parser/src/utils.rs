use std::{fs, path::Path, time::UNIX_EPOCH};

pub fn path_to_modified_time(path: &Path) -> u64 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
