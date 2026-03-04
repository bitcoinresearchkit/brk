use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{finalize_db, open_db},
};

use super::{CountVecs, SpentVecs, Vecs};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 10_000_000)?;
        let version = parent_version;

        let spent = SpentVecs::forced_import(&db, version)?;
        let count = CountVecs::forced_import(&db, version, indexes)?;

        let this = Self { db, spent, count };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
