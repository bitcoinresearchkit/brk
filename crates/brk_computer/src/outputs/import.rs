use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{
        WindowStartVec, Windows,
        db_utils::{finalize_db, open_db},
    },
};

use super::{ByTypeVecs, CountVecs, SpentVecs, Vecs};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 20_000_000)?;
        let version = parent_version;

        let spent = SpentVecs::forced_import(&db, version)?;
        let count = CountVecs::forced_import(&db, version, indexes, cached_starts)?;
        let by_type = ByTypeVecs::forced_import(&db, version, indexes, cached_starts)?;

        let this = Self {
            db,
            spent,
            count,
            by_type,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
