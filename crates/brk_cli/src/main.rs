#![doc = include_str!("../README.md")]

use std::{
    fs,
    path::PathBuf,
    thread::{self, sleep},
    time::Duration,
};

use brk_alloc::Mimalloc;
use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_mempool::Mempool;
use brk_query::AsyncQuery;
use brk_reader::Reader;
use brk_server::{Server, WebsiteSource};
use tracing::info;
use vecdb::Exit;

mod config;
mod paths;
mod website;

use crate::{config::Config, paths::*, website::Website};

pub fn main() -> color_eyre::Result<()> {
    // Can't increase main thread's stack size, thus we need to use another thread
    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

pub fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(dot_brk_path())?;

    brk_logger::init(Some(&dot_brk_log_path()))?;

    let config = Config::import()?;

    let client = config.rpc()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let reader = Reader::new(config.blocksdir(), &client);

    let blocks = Blocks::new(&client, &reader);

    let mut indexer = Indexer::forced_import(&config.brkdir())?;

    let mut computer = Computer::forced_import(&config.brkdir(), &indexer, config.fetcher())?;

    let mempool = Mempool::new(&client);

    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    let query = AsyncQuery::build(&reader, &indexer, &computer, Some(mempool));

    let data_path = config.brkdir();

    let website_source = match config.website() {
        Website::Enabled(false) => WebsiteSource::Disabled,
        Website::Path(p) => WebsiteSource::Filesystem(p),
        Website::Enabled(true) => {
            // Prefer local filesystem if available, otherwise use embedded
            match find_local_website_dir() {
                Some(path) => WebsiteSource::Filesystem(path),
                None => WebsiteSource::Embedded,
            }
        }
    };

    let future = async move {
        let server = Server::new(&query, data_path, website_source);

        tokio::spawn(async move {
            server.serve().await.unwrap();
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

        let starting_indexes = if config.check_collisions() {
            indexer.checked_index(&blocks, &client, &exit)?
        } else {
            indexer.index(&blocks, &client, &exit)?
        };

        Mimalloc::collect();

        computer.compute(&indexer, starting_indexes, &reader, &exit)?;

        info!("Waiting for new blocks...");

        while last_height == client.get_last_height()? {
            sleep(Duration::from_secs(1))
        }
    }
}

/// Path to website directory relative to this crate (only valid at dev machine)
const DEV_WEBSITE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../website");

/// Returns local website path if it exists (dev mode)
fn find_local_website_dir() -> Option<PathBuf> {
    let path = PathBuf::from(DEV_WEBSITE_DIR);
    path.exists().then_some(path)
}
