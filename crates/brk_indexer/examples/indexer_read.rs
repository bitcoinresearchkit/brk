use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Sats;
use std::{fs, path::Path};

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;

    let indexer = Indexer::forced_import(&outputs_dir)?;

    let mut sum = Sats::ZERO;
    let mut count: usize = 0;

    for value in indexer.vecs.txoutindex_to_value.clean_iter().unwrap() {
        sum += value;
        count += 1;
    }

    println!("sum = {sum}, count = {count}");

    Ok(())
}
