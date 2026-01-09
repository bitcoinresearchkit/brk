#![doc = include_str!("../README.md")]

use std::{
    fs,
    io::Cursor,
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
use brk_server::{Server, VERSION};
use tracing::info;
use vecdb::Exit;

mod config;
mod paths;
mod website;

use crate::{config::Config, paths::*};

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

    #[cfg(not(debug_assertions))]
    {
        // Pre-run indexer if too far behind, then drop and reimport to reduce memory
        let chain_height = client.get_last_height()?;
        let indexed_height = indexer.vecs.starting_height();
        if chain_height.saturating_sub(*indexed_height) > 1000 {
            indexer.index(&blocks, &client, &exit)?;
            drop(indexer);
            Mimalloc::collect();
            indexer = Indexer::forced_import(&config.brkdir())?;
        }
    }

    let mut computer = Computer::forced_import(&config.brkdir(), &indexer, config.fetcher())?;

    let mempool = Mempool::new(&client);

    let mempool_clone = mempool.clone();
    thread::spawn(move || {
        mempool_clone.start();
    });

    let query = AsyncQuery::build(&reader, &indexer, &computer, Some(mempool));

    let website = config.website();

    let downloads_path = config.downloads_dir();

    let future = async move {
        let bundle_path = if website.is_some() {
            // Try to find local dev directories - check cwd and parent directories
            let find_dev_dirs = || -> Option<(PathBuf, PathBuf)> {
                let mut dir = std::env::current_dir().ok()?;
                loop {
                    let websites = dir.join("websites");
                    let modules = dir.join("modules");
                    if websites.exists() && modules.exists() {
                        return Some((websites, modules));
                    }
                    // Stop at workspace root (crates/ indicates we're there)
                    if dir.join("crates").exists() {
                        return None;
                    }
                    dir = dir.parent()?.to_path_buf();
                }
            };

            let websites_path = if let Some((websites, _modules)) = find_dev_dirs() {
                websites
            } else {
                let downloaded_brk_path = downloads_path.join(format!("brk-{VERSION}"));
                let downloaded_websites_path = downloaded_brk_path.join("websites");

                if !fs::exists(&downloaded_websites_path)? {
                    info!("Downloading source from Github...");

                    let url = format!(
                        "https://github.com/bitcoinresearchkit/brk/archive/refs/tags/v{VERSION}.zip",
                    );

                    let response = minreq::get(url).with_timeout(60).send()?;
                    let bytes = response.as_bytes();
                    let cursor = Cursor::new(bytes);

                    let mut zip = zip::ZipArchive::new(cursor).unwrap();

                    zip.extract(downloads_path).unwrap();
                }

                downloaded_websites_path
            };

            Some(websites_path.join(website.to_folder_name()))
        } else {
            None
        };

        // Generate import map for cache busting
        if let Some(ref path) = bundle_path {
            match importmap::ImportMap::scan(path, "") {
                Ok(map) => {
                    let html_path = path.join("index.html");
                    if let Ok(html) = fs::read_to_string(&html_path)
                        && let Some(updated) = map.update_html(&html)
                    {
                        let _ = fs::write(&html_path, updated);
                        info!("Updated importmap in index.html");
                    }
                }
                Err(e) => tracing::error!("Failed to generate importmap: {e}"),
            }
        }

        let server = Server::new(&query, bundle_path);

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
