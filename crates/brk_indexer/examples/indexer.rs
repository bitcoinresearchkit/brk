use std::{
    fs,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use vecdb::Exit;

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let bitcoin_dir = Path::new(&std::env::var("HOME").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Bitcoin");
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let client = Client::new(
        "http://localhost:8332",
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), client.clone());

    let blocks = Blocks::new(client.clone(), reader);

    fs::create_dir_all(&outputs_dir)?;

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    // 44
    // let vecs = indexer.vecs.iter_any_collectable().collect::<Vec<_>>();
    // dbg!(indexer.vecs.to_tree_node());
    // dbg!(vecs.len());
    // std::process::exit(0);

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    loop {
        let i = Instant::now();
        indexer.checked_index(&blocks, &client, &exit)?;
        dbg!(i.elapsed());

        sleep(Duration::from_secs(60));
    }
}
