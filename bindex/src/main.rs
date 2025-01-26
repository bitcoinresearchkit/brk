use std::path::Path;

use bindex::Indexer;
use biter::rpc;
use exit::Exit;

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
