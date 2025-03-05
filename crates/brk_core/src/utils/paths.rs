use std::path::{Path, PathBuf};

pub fn path_dot_brk() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    Path::new(&home).join(".brk")
}

pub fn path_dot_brk_log() -> PathBuf {
    path_dot_brk().join("log")
}
