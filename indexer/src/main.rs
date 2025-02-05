use std::path::Path;

use bindex::Indexer;
use exit::Exit;
use iterator::rpc;
use storable_vec::CACHED_GETS;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let data_dir = Path::new("../../bitcoin");
    let rpc = rpc::Client::new(
        "http://localhost:8332",
        rpc::Auth::CookieFile(Path::new(data_dir).join(".cookie")),
    )?;
    let exit = Exit::new();

    let i = std::time::Instant::now();

    let mut indexer: Indexer<CACHED_GETS> = Indexer::import(Path::new("../_outputs/indexes"))?;

    indexer.index(data_dir, rpc, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
