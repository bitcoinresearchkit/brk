use std::path::Path;

use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use vecdb::CacheBudget;

pub fn import_indexer(data_dir: &Path) -> Indexer {
    let bitcoin_dir = Client::default_bitcoin_path();
    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )
    .expect("Failed to connect to Bitcoin Core");
    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);
    let context = ImportContext::new(data_dir, &CACHE_BUDGET);
    Indexer::import(context, &reader).expect("Failed to import indexer")
}

static CACHE_BUDGET: CacheBudget = CacheBudget::new(2 * 1024 * 1024 * 1024);
