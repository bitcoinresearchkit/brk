use std::{thread::sleep, time::Duration};

use bitcoincore_rpc::{self, RpcApi};
use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_server::Server;
use log::info;

use crate::config::Config;

pub fn run() -> color_eyre::Result<()> {
    let config = Config::import()?;

    let rpc = config.rpc()?;
    let exit = Exit::new();
    let parser = brk_parser::Parser::new(
        config.blocksdir(),
        config.brkdir(),
        rpc,
    );

    let format = config.format();

    let mut indexer = Indexer::forced_import(&config.outputsdir())?;

    let wait_for_synced_node = |rpc_client: &bitcoincore_rpc::Client| -> color_eyre::Result<()> {
        let is_synced = || -> color_eyre::Result<bool> {
            let info = rpc_client.get_blockchain_info()?;
            Ok(info.headers == info.blocks)
        };

        if !is_synced()? {
            info!("Waiting for node to be synced...");
            while !is_synced()? {
                sleep(Duration::from_secs(1))
            }
        }

        Ok(())
    };

    let mut computer = Computer::forced_import(
        &config.outputsdir(),
        &indexer,
        config.computation(),
        config.fetcher(),
        format,
    )?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            // Always start the server
            let served_indexer = indexer.clone();
            let served_computer = computer.clone();

            let server = Server::new(served_indexer, served_computer, config.website())?;

            let watch = config.watch();
            let mcp = config.mcp();
            let server_handle = tokio::spawn(async move {
                server.serve(watch, mcp).await.unwrap();
            });

            sleep(Duration::from_secs(1));

            // Always run the processor
            loop {
                wait_for_synced_node(rpc)?;

                let block_count = rpc.get_block_count()?;

                info!("{} blocks found.", block_count + 1);

                let starting_indexes =
                    indexer.index(&parser, rpc, &exit, config.check_collisions())?;

                computer.compute(&mut indexer, starting_indexes, &exit)?;

                if let Some(delay) = config.delay() {
                    sleep(Duration::from_secs(delay))
                }

                info!("Waiting for new blocks...");

                while block_count == rpc.get_block_count()? {
                    sleep(Duration::from_secs(1))
                }
            }
        })
}
