use std::{env, fs, path::Path};

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_query::Vecs;
use vecdb::ReadOnlyClone;

pub fn main() -> brk_error::Result<()> {
    let tmp = env::temp_dir().join("brk_search_gen");
    fs::create_dir_all(&tmp)?;

    let indexer = Indexer::forced_import(&tmp)?;
    let computer = Computer::forced_import(&tmp, &indexer)?;

    let indexer_ro = indexer.read_only_clone();
    let computer_ro = computer.read_only_clone();

    let vecs = Vecs::build(&indexer_ro, &computer_ro);

    let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("series.txt");
    let content = vecs.series.join("\n");
    fs::write(&out_path, &content)?;
    eprintln!(
        "Wrote {} series to {}",
        vecs.series.len(),
        out_path.display()
    );

    fs::remove_dir_all(&tmp)?;

    Ok(())
}
