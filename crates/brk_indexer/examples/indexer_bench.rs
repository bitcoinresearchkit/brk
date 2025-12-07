use std::{
    env, fs,
    path::Path,
    time::{Duration, Instant},
};

use brk_bencher::Bencher;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use log::{debug, info};
use vecdb::Exit;

fn main() -> Result<()> {
    brk_logger::init(None)?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk/benches");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    fs::create_dir_all(&outputs_dir)?;

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    let mut bencher =
        Bencher::from_cargo_env(env!("CARGO_PKG_NAME"), &outputs_dir.join("indexed"))?;
    bencher.start()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();
    let bencher_clone = bencher.clone();
    exit.register_cleanup(move || {
        let _ = bencher_clone.stop();
        debug!("Bench stopped.");
    });

    let i = Instant::now();
    indexer.index(&blocks, &client, &exit)?;
    info!("Done in {:?}", i.elapsed());

    // We want to benchmark the drop too
    drop(indexer);

    std::thread::sleep(Duration::from_secs(10));

    Ok(())
}
