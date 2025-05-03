use std::fs;

use brk_core::{dot_brk_log_path, dot_brk_path};
use brk_query::Params as QueryArgs;
use clap::Parser;
use clap_derive::{Parser, Subcommand};
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

    fs::create_dir_all(dot_brk_path())?;

    brk_logger::init(Some(&dot_brk_log_path()));

    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => run(args),
        Commands::Query(args) => query(args),
    }
}
