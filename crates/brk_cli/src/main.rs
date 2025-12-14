#![doc = include_str!("../README.md")]

use std::{
    fs,
    io::Cursor,
    path::Path,
    thread::{self, sleep},
    time::Duration,
};

use brk_binder::generate_js_files;
use brk_bundler::bundle;
use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_iterator::Blocks;
use brk_mempool::Mempool;
use brk_query::AsyncQuery;
use brk_reader::Reader;
use brk_server::{Server, VERSION};
use log::info;
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
            let websites_dev_path = Path::new("../../websites");
            let modules_dev_path = Path::new("../../modules");

            let websites_path;
            let modules_path;

            if fs::exists(websites_dev_path)? && fs::exists(modules_dev_path)? {
                websites_path = websites_dev_path.to_path_buf();
                modules_path = modules_dev_path.to_path_buf();
            } else {
                let downloaded_brk_path = downloads_path.join(format!("brk-{VERSION}"));

                let downloaded_websites_path = downloaded_brk_path.join("websites");
                let downloaded_modules_path = downloaded_brk_path.join("modules");

                if !fs::exists(&downloaded_websites_path)? {
                    info!("Downloading source from Github...");

                    let url = format!(
                        "https://github.com/bitcoinresearchkit/brk/archive/refs/tags/v{VERSION}.zip",
                    );

                    let response = minreq::get(url).send()?;
                    let bytes = response.as_bytes();
                    let cursor = Cursor::new(bytes);

                    let mut zip = zip::ZipArchive::new(cursor).unwrap();

                    zip.extract(downloads_path).unwrap();
                }

                websites_path = downloaded_websites_path;
                modules_path = downloaded_modules_path;
            }

            generate_js_files(query.inner(), &modules_path)?;

            Some(
                bundle(
                    &modules_path,
                    &websites_path,
                    website.to_folder_name(),
                    true,
                )
                .await?,
            )
        } else {
            None
        };

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

        computer.compute(&indexer, starting_indexes, &reader, &exit)?;

        info!("Waiting for new blocks...");

        while last_height == client.get_last_height()? {
            sleep(Duration::from_secs(1))
        }
    }
}
