use std::{
    env,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_alloc::Mimalloc;
use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use vecdb::Exit;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")))?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("../../_outputs");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    // Pre-run indexer if too far behind, then drop and reimport to reduce memory
    let chain_height = client.get_last_height()?;
    let indexed_height = indexer.vecs.starting_height();
    if u32::from(chain_height).saturating_sub(u32::from(indexed_height)) > 1000 {
        indexer.checked_index(&blocks, &client, &exit)?;
        drop(indexer);
        Mimalloc::collect();
        indexer = Indexer::forced_import(&outputs_dir)?;
    }

    let mut computer = Computer::forced_import(&outputs_dir, &indexer)?;

    loop {
        let i = Instant::now();
        let starting_indexes = indexer.checked_index(&blocks, &client, &exit)?;

        Mimalloc::collect();

        computer.compute(&indexer, starting_indexes, &reader, &exit)?;
        dbg!(i.elapsed());
        sleep(Duration::from_secs(10));
    }
}
