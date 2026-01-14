use std::path::{Path, PathBuf};

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

pub fn fix_user_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/").or(path.strip_prefix("$HOME/"))
        && let Ok(home) = std::env::var("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(path)
}
