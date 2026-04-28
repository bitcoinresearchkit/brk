#![doc = include_str!("../README.md")]

use std::{
    fs,
    thread::{self, sleep},
    time::{Duration, Instant},
};

use brk_alloc::Mimalloc;
use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_query::AsyncQuery;
use brk_reader::Reader;
use brk_server::{Server, ServerConfig};
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
            indexer.index(&reader, &client, &exit)?;
            drop(indexer);
            Mimalloc::collect();
            indexer = Indexer::forced_import(&config.brkdir())?;
        }
    }

    let mut computer = Computer::forced_import(&config.brkdir(), &indexer)?;

    let mempool = Mempool::new(&client);

    let query = AsyncQuery::build(&reader, &indexer, &computer, Some(mempool.clone()));

    let mempool_clone = mempool.clone();
    let query_clone = query.clone();
    thread::spawn(move || {
        mempool_clone.start_with(|| {
            query_clone.sync(|q| q.fill_mempool_prevouts());
        });
    });

    let server_config = ServerConfig {
        data_path: config.brkdir(),
        website: config.website(),
        cdn_cache_mode: config.cdn_cache_mode(),
        max_weight: config.max_weight(),
    };

    let port = config.brkport();

    let future = async move {
        let server = Server::new(&query, server_config);

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

        let total_start = Instant::now();

        let starting_indexes = if cfg!(debug_assertions) {
            indexer.checked_index(&reader, &client, &exit)?
        } else {
            indexer.index(&reader, &client, &exit)?
        };

        Mimalloc::collect();

        computer.compute(&indexer, starting_indexes, &exit)?;

        info!("Total time: {:?}", total_start.elapsed());
        info!("Waiting for new blocks...");

        while last_height == client.get_last_height()? {
            sleep(Duration::from_secs(1))
        }
    }
}
