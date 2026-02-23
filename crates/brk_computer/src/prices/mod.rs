mod compute;

pub mod cents;
pub mod sats;
pub mod usd;

pub use cents::Vecs as CentsVecs;
pub use sats::Vecs as SatsVecs;
pub use usd::Vecs as UsdVecs;

use std::path::Path;

use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, Rw, StorageMode, PAGE_SIZE};

use crate::indexes;

pub const DB_NAME: &str = "prices";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub cents: CentsVecs<M>,
    pub usd: UsdVecs,
    pub sats: SatsVecs,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let db = Database::open(&parent.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let this = Self::forced_import_inner(&db, version, indexes)?;

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }

    fn forced_import_inner(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let cents = CentsVecs::forced_import(db, version, indexes)?;
        let usd = UsdVecs::forced_import(version, indexes, &cents);
        let sats = SatsVecs::forced_import(version, indexes, &cents);

        Ok(Self {
            db: db.clone(),
            cents,
            usd,
            sats,
        })
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}
