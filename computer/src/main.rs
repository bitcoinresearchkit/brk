use std::path::Path;

use bomputer::Computer;
use exit::Exit;
use indexer::Indexer;

mod structs;

pub fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let exit = Exit::new();

    let i = std::time::Instant::now();

    let outputs_dir = Path::new("../_outputs");

    Computer::import(&outputs_dir.join("computed"))?.compute(Indexer::import(&outputs_dir.join("indexes"))?, &exit)?;

    dbg!(i.elapsed());

    Ok(())
}
