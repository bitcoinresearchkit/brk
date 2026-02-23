#![doc = include_str!("../README.md")]

use std::{
    fs,
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
use brk_server::Server;
use tracing::info;
use vecdb::Exit;

mod config;
mod paths;

use crate::{config::Config, paths::*};

pub fn main() -> anyhow::Result<()> {
    fs::create_dir_all(dot_brk_path())?;

    brk_logger::init(Some(&dot_brk_log_path()))?;

    let config = Config::import()?;

    let client = config.rpc()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let reader = Reader::new(config.blocksdir(), &client);

    let blocks = Blocks::new(&client, &reader);

    let mut indexer = Indexer::forced_import(&config.brkdir())?;

    #[cfg(not(debug_assertions))]
    {
        // Pre-run indexer if too far behind, then drop and reimport to reduce memory
        let chain_height = client.get_last_height()?;
        let indexed_height = indexer.vecs.starting_height();
        let blocks_behind = chain_height.saturating_sub(*indexed_height);
        if blocks_behind > 10_000 {
            info!("---");
            info!("Indexing {blocks_behind} blocks before starting server...");
            info!("---");
            sleep(Duration::from_secs(10));
            indexer.index(&blocks, &client, &exit)?;
            drop(indexer);
            Mimalloc::collect();
            indexer = Indexer::forced_import(&config.brkdir())?;
        }
    }

    let mut computer = Computer::forced_import(&config.brkdir(), &indexer)?;

    let mempool = Mempool::new(&client);

    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    let query = AsyncQuery::build(&reader, &indexer, &computer, Some(mempool));

    let data_path = config.brkdir();

    let website = config.website();

    let port = config.brkport();

    let future = async move {
        let server = Server::new(&query, data_path, website);

        tokio::spawn(async move {
            server.serve(port).await.unwrap();
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

        let starting_indexes = if cfg!(debug_assertions) {
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
