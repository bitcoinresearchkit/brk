use std::path::Path;

use bindex::{biter::rpc, Indexer};
use exit::Exit;

mod structs;

use structs::*;

pub struct Bomputer;

impl Bomputer {
    pub fn compute() {}
}

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data_dir = Path::new("../../bitcoin");
    let rpc = rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?;
    let exit = Exit::new();

    let i = std::time::Instant::now();

    let mut indexer = Indexer::import(Path::new("indexes"))?;

    indexer.index(data_dir, rpc, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
