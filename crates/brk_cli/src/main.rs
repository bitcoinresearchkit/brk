use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(RunArgs),
    Query(QueryArgs),
}

#[derive(Args)]
struct RunArgs {
    name: Option<String>,
}

#[derive(Args)]
struct QueryArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Run(name) => {
            println!("'myapp add' was used, name is: {:?}", name.name);
        }
        Commands::Query(name) => {
            println!("'myapp add' was used, name is: {:?}", name.name);
        }
    }
}
