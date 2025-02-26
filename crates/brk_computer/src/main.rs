use std::path::Path;

use brk_computer::Computer;
use brk_indexer::Indexer;
use hodor::Hodor;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let hodor = Hodor::new();

    let i = std::time::Instant::now();

    let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::import(&outputs_dir.join("indexes"))?;

    let mut computer = Computer::import(&outputs_dir.join("computed"))?;

    computer.compute(indexer, &hodor)?;

    dbg!(i.elapsed());

    Ok(())
}
