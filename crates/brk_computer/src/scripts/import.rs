use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{finalize_db, open_db},
};

use super::{AdoptionVecs, CountVecs, ValueVecs, Vecs};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 50_000_000)?;
        let version = parent_version;

        let count = CountVecs::forced_import(&db, version, indexes)?;
        let value = ValueVecs::forced_import(&db, version, indexes)?;
        let adoption = AdoptionVecs::forced_import(&db, version, indexes)?;

        let this = Self {
            db,
            count,
            value,
            adoption,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
