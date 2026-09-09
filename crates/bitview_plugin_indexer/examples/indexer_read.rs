use std::{env, fs, path::Path};

use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use brk_error::Result;
use brk_logger::init;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use vecdb::{CacheBudget, ReadableVec};

fn main() -> Result<()> {
    init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&env::var("HOME").unwrap()).join(".bitview");
    fs::create_dir_all(&outputs_dir)?;

    let bitcoin_dir = Client::default_bitcoin_path();
    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;
    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);
    let context = ImportContext::new(&outputs_dir, &CACHE_BUDGET);
    let indexer = Indexer::import(context, &reader)?;

    println!(
        "{:?}",
        indexer.vecs().outputs.value.collect_range_at(0, 200)
    );

    Ok(())
}

static CACHE_BUDGET: CacheBudget = CacheBudget::new(2 * 1024 * 1024 * 1024);
