use std::{path::Path, thread};

use brk_computer::Computer;
use brk_core::{default_bitcoin_path, default_brk_path};
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_vec::Computation;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    // Can't increase main thread's stack programatically, thus we need to use another thread
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || -> color_eyre::Result<()> {
            let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

            let _outputs_dir = default_brk_path().join("outputs");
            let outputs_dir = _outputs_dir.as_path();
            // let outputs_dir = Path::new("../../_outputs");

            let compressed = false;

            let mut indexer = Indexer::new(outputs_dir, compressed, true)?;
            indexer.import_stores()?;
            indexer.import_vecs()?;

            let fetcher = Fetcher::import(None)?;

            let mut computer = Computer::new(outputs_dir, Some(fetcher), compressed);
            computer.import_stores(&indexer)?;
            computer.import_vecs(&indexer, Computation::Lazy)?;

            let starting_indexes = indexer.index(&parser, rpc, &exit)?;

            computer.compute(&mut indexer, starting_indexes, &exit)?;

            Ok(())
        })?
        .join()
        .unwrap()
}
