use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Query};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::import(&outputs_dir.join("indexed"))?;

    let computer = Computer::import(&outputs_dir.join("computed"))?;

    let query = Query::build(&indexer, &computer);

    dbg!(query.search(Index::Height, &["date"], Some(-1), None, None)?);
    dbg!(query.search(Index::Height, &["date"], Some(-10), None, None)?);
    dbg!(query.search(Index::Height, &["date", "timestamp"], Some(-10), None, None)?);

    Ok(())
}
