use std::fs;

use brk_core::{path_dot_brk, path_dot_brk_log};
use brk_query::Params as QueryArgs;
use clap::{Parser, Subcommand};
use query::query;
use run::{RunConfig, run};

mod query;
mod run;

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the indexer, computer and server
    Run(RunConfig),
    /// Query generated datasets via the `run` command in a similar fashion as the server's API
    Query(QueryArgs),
}

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(path_dot_brk())?;

    brk_logger::init(Some(&path_dot_brk_log()));

    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => run(args),
        Commands::Query(args) => query(args),
    }
}
