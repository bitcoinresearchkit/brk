use std::{
    env, fs,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_alloc::Mimalloc;
use brk_bencher::Bencher;
use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use tracing::{debug, info};
use vecdb::Exit;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");
    fs::create_dir_all(&outputs_dir)?;

    brk_logger::init(Some(&outputs_dir.join("log")))?;

    let mut bencher = Bencher::from_cargo_env("brk", &outputs_dir)?;
    bencher.start()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();
    let bencher_clone = bencher.clone();
    exit.register_cleanup(move || {
        let _ = bencher_clone.stop();
        debug!("Bench stopped.");
    });

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    // Pre-run indexer if too far behind, then drop and reimport to reduce memory
    let chain_height = client.get_last_height()?;
    let indexed_height = indexer.vecs.starting_height();
    if chain_height.saturating_sub(*indexed_height) > 1000 {
        indexer.index(&blocks, &client, &exit)?;
        drop(indexer);
        Mimalloc::collect();
        indexer = Indexer::forced_import(&outputs_dir)?;
    }

    let mut computer = Computer::forced_import(&outputs_dir, &indexer)?;

    loop {
        let i = Instant::now();
        let starting_indexes = indexer.index(&blocks, &client, &exit)?;
        info!("Done in {:?}", i.elapsed());

        Mimalloc::collect();

        let i = Instant::now();
        computer.compute(&indexer, starting_indexes, &reader, &exit)?;
        info!("Done in {:?}", i.elapsed());

        sleep(Duration::from_secs(60));
    }
}
