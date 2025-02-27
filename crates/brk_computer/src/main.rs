use std::path::Path;

use brk_computer::Computer;
use brk_exit::Exit;
use brk_indexer::Indexer;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let exit = Exit::new();

    let i = std::time::Instant::now();

    let outputs_dir = Path::new("../../_outputs");

    let indexer = Indexer::import(&outputs_dir.join("indexes"))?;

    let mut computer = Computer::import(&outputs_dir.join("computed"))?;

    computer.compute(indexer, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
