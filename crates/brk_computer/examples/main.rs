use std::path::Path;

use brk_computer::{Computation, Computer};
use brk_core::default_bitcoin_path;
use brk_exit::Exit;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_parser::{Parser, rpc};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let outputs_dir = Path::new("../../_outputs");

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
}
