mod analysis;
mod group;
mod input;
mod record;
mod report;
mod routes;

use std::{io::Write, process::ExitCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    if std::env::args_os().len() != 1 {
        return Err("no arguments supported; paths come from bitviewd::Config".into());
    }
    let config = bitviewd::Config::load()?.server_config();
    let parent = &config.data_path;
    let files = input::discover(&config.logs_path())?;
    let analysis = analysis::Analysis::read(&files)?;
    let report = report::render(&analysis)?;
    let path = parent.join("latency.md");
    // Replace the report without following an existing symlink or truncating a
    // hard-linked source file. A failed write leaves the previous report intact.
    let mut output = tempfile::NamedTempFile::new_in(parent)?;
    output.write_all(report.as_bytes())?;
    output.persist(&path)?;
    println!("Report written to {}", path.display());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("bitviewd_latency: {e}");
            ExitCode::FAILURE
        }
    }
}
