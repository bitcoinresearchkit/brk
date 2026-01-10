use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::{indexes, price};

use super::{CountVecs, FeesVecs, SizeVecs, Vecs, VersionsVecs, VolumeVecs};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version;

        let count = CountVecs::forced_import(&db, version, indexer, indexes)?;
        let size = SizeVecs::forced_import(&db, version, indexer, indexes)?;
        let fees = FeesVecs::forced_import(&db, version, indexer, indexes, price)?;
        let versions = VersionsVecs::forced_import(&db, version, indexes)?;
        let volume = VolumeVecs::forced_import(&db, version, indexes, price)?;

        let this = Self {
            db,
            count,
            size,
            fees,
            versions,
            volume,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }
}
