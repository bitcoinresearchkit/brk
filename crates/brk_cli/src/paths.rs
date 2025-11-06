use std::{
    env,
    path::{Path, PathBuf},
};

pub fn dot_brk_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    Path::new(&home).join(".brk")
}

pub fn dot_brk_log_path() -> PathBuf {
    dot_brk_path().join("log")
}

pub fn default_brk_path() -> PathBuf {
    dot_brk_path()
}
