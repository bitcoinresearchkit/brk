use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use vecdb::{Database, PAGE_SIZE};

pub(crate) fn open_db(
    parent_path: &Path,
    db_name: &str,
    page_multiplier: usize,
) -> Result<Database> {
    let db = Database::open(&parent_path.join(db_name))?;
    db.set_min_len(PAGE_SIZE * page_multiplier)?;
    Ok(db)
}

pub(crate) fn finalize_db(db: &Database, traversable: &impl Traversable) -> Result<()> {
    db.retain_regions(
        traversable
            .iter_any_exportable()
            .flat_map(|v| v.region_names())
            .collect(),
    )?;
    db.compact()?;
    Ok(())
}
