use std::{
    fs,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::Parser;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    // let bitcoin_dir = brk_core::default_bitcoin_path();
    let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");
    // let outputs_dir = brk_core::default_brk_path().join("outputs");
    let outputs_dir = Path::new("/Volumes/WD_BLACK/brk/outputs");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));
    let exit = Exit::new();

    let parser = Parser::new(bitcoin_dir.join("blocks"), outputs_dir.to_path_buf(), rpc);

    fs::create_dir_all(outputs_dir)?;

    let mut indexer = Indexer::forced_import(outputs_dir)?;

    loop {
        let i = Instant::now();
        indexer.index(&parser, rpc, &exit, false)?;
        dbg!(i.elapsed());

        sleep(Duration::from_secs(5 * 60));
    }
}
