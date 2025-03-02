use std::{path::Path, thread::sleep, time::Duration};

use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_parser::rpc::{self, RpcApi};
use brk_query::{Index, Params as QueryParams, Query};
use brk_server::tokio;
use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
    Query(QueryParams),
}

#[derive(Parser, Debug)]
struct RunArgs {
    name: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let cli = Cli::parse();

    let outputs_dir = Path::new("../../_outputs");

    let mut indexer = Indexer::import(&outputs_dir.join("indexed"))?;

    let mut computer = Computer::import(&outputs_dir.join("computed"))?;

    match &cli.command {
        Commands::Run(args) => {
            let data_dir = Path::new("../../../bitcoin");
            let rpc = Box::leak(Box::new(rpc::Client::new(
                "http://localhost:8332",
                rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
            )?));
            let exit = Exit::new();

            let parser = brk_parser::Parser::new(data_dir, rpc);

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?
                .block_on(async {
                    let served_indexer = indexer.clone();
                    let served_computer = computer.clone();

                    tokio::spawn(async move {
                        brk_server::main(served_indexer, served_computer).await.unwrap();
                    });

                    loop {
                        let block_count = rpc.get_block_count()?;

                        info!("{block_count} blocks found.");

                        let starting_indexes = indexer.index(&parser, rpc, &exit)?;

                        computer.compute(&mut indexer, starting_indexes, &exit)?;

                        info!("Waiting for new blocks...");

                        while block_count == rpc.get_block_count()? {
                            sleep(Duration::from_secs(1))
                        }
                    }

                    #[allow(unreachable_code)]
                    Ok(())
                })
        }
        Commands::Query(args) => {
            let query = Query::build(&indexer, &computer);

            println!(
                "{}",
                query.search(
                    Index::try_from(args.index.as_str())?,
                    &args.values.iter().flat_map(|v| v.split(",")).collect::<Vec<_>>(),
                    args.from,
                    args.to,
                    args.format
                )?
            );

            Ok(())
        }
    }
}
