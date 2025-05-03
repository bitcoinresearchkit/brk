use std::path::Path;

use brk_computer::{Computation, Computer};
use brk_indexer::Indexer;
use brk_query::{Index, Query};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let compressed = true;

    let mut indexer = Indexer::new(outputs_dir, compressed, true)?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(outputs_dir, None, compressed);
    computer.import_vecs(&indexer, Computation::Lazy)?;

    let query = Query::build(&indexer, &computer);

    dbg!(query.search_and_format(Index::Height, &["date"], Some(-1), None, None)?);
    dbg!(query.search_and_format(Index::Height, &["date"], Some(-10), None, None)?);
    dbg!(query.search_and_format(Index::Height, &["date", "timestamp"], Some(-10), None, None)?);

    Ok(())
}
