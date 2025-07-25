use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_interface::{Index, Interface, Params, ParamsOpt};
use brk_vecs::{Computation, Format};

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let outputs_dir = Path::new("../../_outputs");

    let format = Format::Compressed;

    let indexer = Indexer::forced_import(outputs_dir)?;

    let computer = Computer::forced_import(outputs_dir, &indexer, Computation::Lazy, None, format)?;

    let interface = Interface::build(&indexer, &computer);

    dbg!(interface.search_and_format(Params {
        index: Index::Height,
        ids: vec!["date"].into(),
        rest: ParamsOpt::default().set_from(-1),
    })?);
    dbg!(interface.search_and_format(Params {
        index: Index::Height,
        ids: vec!["date", "timestamp"].into(),
        rest: ParamsOpt::default().set_from(-10).set_count(5),
    })?);

    Ok(())
}
