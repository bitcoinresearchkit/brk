use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::{indexes, outputs, price};

use super::{CountVecs, ValueVecs, Vecs};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        outputs: &outputs::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version + Version::ZERO;
        let compute_dollars = price.is_some();

        let count = CountVecs::forced_import(&db, version, indexes, outputs)?;
        let value = ValueVecs::forced_import(&db, version, indexes, compute_dollars)?;

        let this = Self { db, count, value };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }
}
