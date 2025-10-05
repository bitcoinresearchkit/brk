use std::{
    fs,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_parser::Parser;
// use brk_vecs::IVecs;
use vecdb::Exit;

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let blocks_dir = bitcoin_dir.join("blocks");

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let rpc = Box::leak(Box::new(bitcoincore_rpc::Client::new(
        "http://localhost:8332",
        bitcoincore_rpc::Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?));

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let parser = Parser::new(blocks_dir, rpc);

    fs::create_dir_all(&outputs_dir)?;

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    // let vecs = indexer.vecs.iter().collect::<Vec<_>>();
    // dbg!(indexer.vecs.to_tree_node());
    // dbg!(vecs.len());
    // std::process::exit(0);

    loop {
        let i = Instant::now();
        indexer.index(&parser, rpc, &exit, true)?;
        dbg!(i.elapsed());

        sleep(Duration::from_secs(5 * 60));
    }
}
