use std::{path::Path, time::Instant};

use brk_core::default_bitcoin_path;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let i = Instant::now();

    brk_logger::init(Some(Path::new(".log")));

    let bitcoin_dir = default_bitcoin_path();

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), rpc);

    let outputs = Path::new("../../_outputs");

    let mut indexer = Indexer::new(outputs, false, false)?;

    indexer.import_stores()?;
    indexer.import_vecs()?;

    indexer.index(&parser, rpc, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
