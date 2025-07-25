use std::{path::Path, thread};

use brk_computer::Computer;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_vecs::{Computation, Format};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    // let bitcoin_dir = brk_core::default_bitcoin_path();
    let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    // Can't increase main thread's stack programatically, thus we need to use another thread
    thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || -> color_eyre::Result<()> {
            let parser = Parser::new(
                bitcoin_dir.join("blocks"),
                brk_core::default_brk_path(),
                rpc,
            );

            let _outputs_dir = Path::new("/Volumes/WD_BLACK/brk").join("outputs");
            let outputs_dir = _outputs_dir.as_path();
            // let outputs_dir = Path::new("../../_outputs");

            let format = Format::Raw;

            let mut indexer = Indexer::forced_import(outputs_dir)?;

            let fetcher = Fetcher::import(None)?;

            let mut computer = Computer::forced_import(
                outputs_dir,
                &indexer,
                Computation::Lazy,
                Some(fetcher),
                format,
            )?;

            let starting_indexes = indexer.index(&parser, rpc, &exit, true)?;

            computer.compute(&mut indexer, starting_indexes, &exit)?;

            Ok(())
        })?
        .join()
        .unwrap()
}
