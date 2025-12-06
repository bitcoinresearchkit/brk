use std::{
    path::Path,
    thread::{self, sleep},
    time::Duration,
};

use brk_computer::Computer;

use brk_error::Result;
use brk_fetcher::Fetcher;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_query::AsyncQuery;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_server::Server;
use log::info;
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
    brk_logger::init(Some(Path::new(".log")))?;

    let bitcoin_dir = Client::default_bitcoin_path();
    // let bitcoin_dir = Path::new("/Volumes/WD_BLACK1/bitcoin");

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    // let outputs_dir = Path::new("../../_outputs");

    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;

    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let blocks = Blocks::new(&client, &reader);

    let mut indexer = Indexer::forced_import(&outputs_dir)?;

    let fetcher = Some(Fetcher::import(true, None)?);

    let mut computer = Computer::forced_import(&outputs_dir, &indexer, fetcher)?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let query = AsyncQuery::build(&reader, &indexer, &computer, None);

    let future = async move {
        let server = Server::new(&query, None);

        tokio::spawn(async move {
            server.serve(true).await.unwrap();
        });

        Ok(()) as Result<()>
    };

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let _handle = runtime.spawn(future);

    loop {
        client.wait_for_synced_node()?;

        let last_height = client.get_last_height()?;

        info!("{} blocks found.", u32::from(last_height) + 1);

        let starting_indexes = indexer.checked_index(&blocks, &client, &exit)?;

        computer.compute(&indexer, starting_indexes, &reader, &exit)?;

        info!("Waiting for new blocks...");

        while last_height == client.get_last_height()? {
            sleep(Duration::from_secs(1))
        }
    }
}
