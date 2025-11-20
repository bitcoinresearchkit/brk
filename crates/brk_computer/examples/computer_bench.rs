use std::{env, path::Path, thread, time::Instant};

use brk_bencher::Bencher;
use brk_computer::Computer;
use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::{Indexer, Indexes};
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use log::{debug, info};
use vecdb::Exit;

pub fn main() -> Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

fn run() -> Result<()> {
    brk_logger::init(None)?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK/bitcoin");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    let outputs_benches_dir = outputs_dir.join("benches");
    // let outputs_dir = Path::new("../../_outputs");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let fetcher = Fetcher::import(true, None)?;

    let mut computer = Computer::forced_import(&outputs_benches_dir, &indexer, Some(fetcher))?;

    let mut bencher =
        Bencher::from_cargo_env(env!("CARGO_PKG_NAME"), &outputs_dir.join("computed"))?;
    bencher.start()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();
    let bencher_clone = bencher.clone();
    exit.register_cleanup(move || {
        let _ = bencher_clone.stop();
        debug!("Bench stopped.");
    });

    let i = Instant::now();
    // let starting_indexes = indexer.checked_index(&blocks, &client, &exit)?;
    let starting_indexes = Indexes::default();
    info!("Done in {:?}", i.elapsed());

    let i = Instant::now();
    computer.compute(&indexer, starting_indexes, &reader, &exit)?;
    info!("Done in {:?}", i.elapsed());

    // We want to benchmark the drop too
    drop(computer);
    drop(indexer);

    Ok(())
}
