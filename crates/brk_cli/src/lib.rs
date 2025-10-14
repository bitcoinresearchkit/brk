#![doc = include_str!("../README.md")]

use std::{
    fs,
    io::Cursor,
    path::Path,
    thread::{self, sleep},
    time::Duration,
};

use bitcoincore_rpc::{self, RpcApi};
use brk_binder::Bridge;
use brk_bundler::bundle;
use brk_computer::Computer;
use brk_error::Result;
use brk_indexer::Indexer;
use brk_interface::Interface;
use brk_reader::Reader;
use brk_server::{Server, VERSION};
use log::info;
use vecdb::Exit;

mod config;
mod paths;
mod website;

use crate::{config::Config, paths::*};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(dot_brk_path())?;

    brk_logger::init(Some(&dot_brk_log_path()))?;

    thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

pub fn run() -> color_eyre::Result<()> {
    let config = Config::import()?;

    let rpc = config.rpc()?;

    let exit = Exit::new();
    exit.set_ctrlc_handler();

    let parser = Reader::new(config.blocksdir(), rpc);

    let mut indexer = Indexer::forced_import(&config.brkdir())?;

    let mut computer = Computer::forced_import(&config.brkdir(), &indexer, config.fetcher())?;

    let interface = Interface::build(&parser, &indexer, &computer);

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

            interface.generate_js_files(&modules_path)?;

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

        let server = Server::new(interface, bundle_path);

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
        wait_for_synced_node(rpc)?;

        let block_count = rpc.get_block_count()?;

        info!("{} blocks found.", block_count + 1);

        let starting_indexes = indexer
            .index(&parser, rpc, &exit, config.check_collisions())
            .unwrap();

        computer
            .compute(&indexer, starting_indexes, &parser, &exit)
            .unwrap();

        info!("Waiting for new blocks...");

        while block_count == rpc.get_block_count()? {
            sleep(Duration::from_secs(1))
        }
    }
}

fn wait_for_synced_node(rpc_client: &bitcoincore_rpc::Client) -> color_eyre::Result<()> {
    let is_synced = || -> color_eyre::Result<bool> {
        let info = rpc_client.get_blockchain_info()?;
        Ok(info.headers == info.blocks)
    };

    if !is_synced()? {
        info!("Waiting for node to sync...");
        while !is_synced()? {
            sleep(Duration::from_secs(1))
        }
    }

    Ok(())
}
