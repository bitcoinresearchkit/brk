use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::{Index, Query};
use brk_vec::{Computation, Format};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let format = Format::Compressed;

    let mut indexer = Indexer::new(outputs_dir, true)?;
    indexer.import_vecs()?;

    let mut computer = Computer::new(outputs_dir, None, format);
    computer.import_vecs(&indexer, Computation::Lazy)?;

    let query = Query::build(&indexer, &computer);

    dbg!(query.search_and_format(Index::Height, &["date"], Some(-1), None, None)?);
    dbg!(query.search_and_format(Index::Height, &["date"], Some(-10), None, None)?);
    dbg!(query.search_and_format(Index::Height, &["date", "timestamp"], Some(-10), None, None)?);

    Ok(())
}
