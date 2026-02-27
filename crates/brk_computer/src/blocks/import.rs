use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::indexes;

use super::{
    CountVecs, DifficultyVecs, HalvingVecs, IntervalVecs, SizeVecs,
    TimeVecs, Vecs, WeightVecs,
};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version;

        let count = CountVecs::forced_import(&db, version, indexes)?;
        let interval = IntervalVecs::forced_import(&db, version, indexes)?;
        let size = SizeVecs::forced_import(&db, version, indexes)?;
        let weight = WeightVecs::forced_import(&db, version, indexes)?;
        let time = TimeVecs::forced_import(&db, version, indexes)?;
        let difficulty = DifficultyVecs::forced_import(&db, version, indexer, indexes)?;
        let halving = HalvingVecs::forced_import(&db, version, indexes)?;

        let this = Self {
            db,
            count,
            interval,
            size,
            weight,
            time,
            difficulty,
            halving,
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
