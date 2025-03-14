use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Query};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let compressed = true;

    let mut indexer = Indexer::new(outputs_dir.join("indexed"), compressed, true)?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(outputs_dir.join("computed"), None, compressed);
    computer.import_vecs()?;

    let query = Query::build(&indexer, &computer);

    dbg!(query.search(Index::Height, &["date"], Some(-1), None, None)?);
    dbg!(query.search(Index::Height, &["date"], Some(-10), None, None)?);
    dbg!(query.search(Index::Height, &["date", "timestamp"], Some(-10), None, None)?);

    Ok(())
}
