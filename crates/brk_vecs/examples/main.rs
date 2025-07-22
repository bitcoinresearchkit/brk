use std::path::Path;

use brk_core::Result;
use brk_vecs::{File, PAGE_SIZE};

fn main() -> Result<()> {
    let file = File::open(Path::new("vecs"))?;

    file.set_min_len(PAGE_SIZE * 1_000_000)?;

    Ok(())
}
