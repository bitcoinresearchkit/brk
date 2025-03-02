use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::Params as QueryArgs;
use clap::{Parser, Subcommand};
use query::query;
use run::{RunArgs, run};

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
    Run(RunArgs),
    /// Query generated datasets via the `run` command in a similar fashion as the server's API
    Query(QueryArgs),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    brk_logger::init(Some(Path::new(".log")));

    let cli = Cli::parse();

    let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::import(&outputs_dir.join("indexed"))?;

    let computer = Computer::import(&outputs_dir.join("computed"))?;

    match &cli.command {
        Commands::Run(_) => run(indexer, computer),
        Commands::Query(args) => query(indexer, computer, args),
    }
}
