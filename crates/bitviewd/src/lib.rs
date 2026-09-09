#![doc = include_str!("../README.md")]

use bitview::{ComputePluginSet, ImportContext, QueryPluginSet, run as BitviewRun};
use brk_error::Result;
use brk_exit::Exit;
use brk_logger::init;
use brk_reader::Reader;
use vecdb::ReadOnlyClone;

mod config;
mod paths;

pub use config::Config;

/// Runs the Bitview daemon process with the supplied composition.
pub fn run<P>(import: impl FnMut(ImportContext<'_>, &Reader) -> Result<P>) -> Result<()>
where
    P: ComputePluginSet + ReadOnlyClone,
    P::ReadOnly: QueryPluginSet + 'static,
{
    let config = Config::import()?;

    init(Some(&config.server.logs_path()))?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    BitviewRun(config, exit, import)
}
