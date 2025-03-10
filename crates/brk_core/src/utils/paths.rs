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

pub fn default_bitcoin_path() -> PathBuf {
    if env::consts::OS == "macos" {
        default_mac_bitcoin_path()
    } else {
        default_linux_bitcoin_path()
    }
}

pub fn default_linux_bitcoin_path() -> PathBuf {
    Path::new(&std::env::var("HOME").unwrap()).join(".bitcoin")
}

pub fn default_mac_bitcoin_path() -> PathBuf {
    Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin")
}
