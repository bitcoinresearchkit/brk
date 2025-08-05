use std::{path::Path, thread};

use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::Parser;
use brk_vecs::Exit;

pub fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();
    exit.set_ctrlc_handler();

    // Can't increase main thread's stack programatically, thus we need to use another thread
    thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || -> Result<()> {
            let outputs_dir = Path::new("../../_outputs");

            let parser = Parser::new(bitcoin_dir.join("blocks"), outputs_dir.to_path_buf(), rpc);

            let mut indexer = Indexer::forced_import(outputs_dir)?;

            let fetcher = Fetcher::import(None)?;

            let mut computer = Computer::forced_import(outputs_dir, &indexer, Some(fetcher))?;

            let starting_indexes = indexer.index(&parser, rpc, &exit, true)?;

            computer.compute(&indexer, starting_indexes, &exit)?;

            Ok(())
        })?
        .join()
        .unwrap()
}
