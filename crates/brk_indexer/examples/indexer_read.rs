use std::{fs, path::Path};

use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::ReadableVec;

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;

    let indexer = Indexer::forced_import(&outputs_dir)?;

    println!("{:?}", indexer.vecs.outputs.value.collect_range(0, 200));

    Ok(())
}
