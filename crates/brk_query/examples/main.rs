use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Query};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let mut indexer = Indexer::new(outputs_dir.join("indexed"))?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(outputs_dir.join("computed"), None);
    computer.import_vecs()?;

    let query = Query::build(&indexer, &computer);

    dbg!(query.search(Index::Height, &["date"], Some(-1), None, None)?);
    dbg!(query.search(Index::Height, &["date"], Some(-10), None, None)?);
    dbg!(query.search(Index::Height, &["date", "timestamp"], Some(-10), None, None)?);

    Ok(())
}
