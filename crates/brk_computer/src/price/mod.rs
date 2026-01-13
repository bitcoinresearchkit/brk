mod compute;
mod fetch;

pub mod cents;
pub mod oracle;
pub mod sats;
pub mod usd;

pub use cents::Vecs as CentsVecs;
pub use oracle::Vecs as OracleVecs;
pub use sats::Vecs as SatsVecs;
pub use usd::Vecs as UsdVecs;

use std::path::Path;

use brk_fetcher::Fetcher;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, PAGE_SIZE};

use crate::indexes;

pub const DB_NAME: &str = "price";

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    #[traversable(skip)]
    pub(crate) fetcher: Option<Fetcher>,

    pub cents: CentsVecs,
    pub usd: UsdVecs,
    pub sats: SatsVecs,
    pub oracle: OracleVecs,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
        fetcher: Option<Fetcher>,
    ) -> brk_error::Result<Self> {
        let db = Database::open(&parent.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let this = Self::forced_import_inner(&db, version, indexes, fetcher)?;

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
        fetcher: Option<Fetcher>,
    ) -> brk_error::Result<Self> {
        let cents = CentsVecs::forced_import(db, version)?;
        let usd = UsdVecs::forced_import(db, version, indexes)?;
        let sats = SatsVecs::forced_import(db, version, indexes)?;
        let oracle = OracleVecs::forced_import(db, version)?;

        Ok(Self {
            db: db.clone(),
            fetcher,
            cents,
            usd,
            sats,
            oracle,
        })
    }

    pub fn has_fetcher(&self) -> bool {
        self.fetcher.is_some()
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}
