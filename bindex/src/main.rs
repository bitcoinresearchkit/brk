use std::path::Path;

use biter::rpc;

mod indexer;
mod storage;
mod structs;

use exit::Exit;
use indexer::Indexer;
use structs::{AddressbytesPrefix, Addressindex, BlockHashPrefix, Height, TxidPrefix, Txindex, Txoutindex};

// https://github.com/romanz/electrs/blob/master/doc/schema.md

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data_dir = Path::new("../../bitcoin");
    let rpc = rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?;
    let exit = Exit::new();

    let i = std::time::Instant::now();

    Indexer::index(Path::new("indexes"), data_dir, rpc, exit)?;

    dbg!(i.elapsed());

    Ok(())
}
