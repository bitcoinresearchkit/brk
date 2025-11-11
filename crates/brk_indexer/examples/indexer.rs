use std::{
    env, fs,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use log::{debug, info};
use vecdb::Exit;

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;
    // let outputs_dir = Path::new("/Volumes/WD_BLACK1/brk");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);
    debug!("Reader created.");

    let blocks = Blocks::new(&client, &reader);
    debug!("Blocks created.");

    fs::create_dir_all(&outputs_dir)?;

    let mut indexer = Indexer::forced_import(&outputs_dir)?;
    debug!("Indexer imported.");

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    loop {
        let i = Instant::now();
        indexer.checked_index(&blocks, &client, &exit)?;
        info!("Done in {:?}", i.elapsed());

        sleep(Duration::from_secs(60));
    }
}
