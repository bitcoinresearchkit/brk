use std::{
    env,
    path::{Path, PathBuf},
};

pub fn path_dot_brk() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    Path::new(&home).join(".brk")
}

pub fn path_dot_brk_log() -> PathBuf {
    path_dot_brk().join("log")
}

pub fn default_brk() -> PathBuf {
    path_dot_brk()
}

pub fn default_bitcoin_path() -> PathBuf {
    if env::consts::OS == "macos" {
        default_mac_bitcoin_path()
    } else {
        default_linux_bitcoin_path()
    }
}

fn default_linux_bitcoin_path() -> PathBuf {
    Path::new(&std::env::var("HOME").unwrap()).join(".bitcoin")
}

fn default_mac_bitcoin_path() -> PathBuf {
    Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin")
}
