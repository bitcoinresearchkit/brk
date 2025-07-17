#![doc = include_str!("../README.md")]

use std::{fs, thread};

use brk_core::{dot_brk_log_path, dot_brk_path};

mod config;
mod run;

use run::*;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(dot_brk_path())?;

    brk_logger::init(Some(&dot_brk_log_path()));

    thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
