use biter::bitcoincore_rpc::Client;
use log::info;
use rlimit::{getrlimit, setrlimit, Resource};

mod io;
mod parser;
mod server;
mod structs;
mod utils;

use parser::Datasets;
use server::api::structs::Routes;
use structs::{Config, Exit};
use utils::init_log;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    init_log();

    let (_, nofile_limit) = getrlimit(Resource::NOFILE).unwrap();
    setrlimit(Resource::NOFILE, 138_240, nofile_limit)?;

    std::thread::Builder::new()
        .stack_size(getrlimit(Resource::STACK).unwrap().1 as usize)
        .spawn(|| -> color_eyre::Result<()> {
            let exit = Exit::new();

            let config = Config::import()?;

            info!("Starting...");

            let rpc = Client::from(&config);

            let routes = Routes::build(&Datasets::import(&config)?, &config);

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let run_parser = config.parser();
                    let run_server = config.server();

                    let config_clone = config.clone();
                    let handle = tokio::spawn(async move {
                        if run_server {
                            server::main(routes, config_clone).await.unwrap();
                        } else {
                            info!("Skipping server");
                        }
                    });

                    if run_parser {
                        parser::main(&config, &rpc, &exit)?;
                    } else {
                        info!("Skipping parser");
                    }

                    handle.await?;

                    Ok(())
                })
        })?
        .join()
        .unwrap()
}
