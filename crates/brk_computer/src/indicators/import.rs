use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use super::Vecs;
use crate::{
    indexes,
    internal::{finalize_db, open_db, PercentPerBlock, RatioPerBlock},
};

const VERSION: Version = Version::new(1);

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 100_000)?;
        let v = parent_version + VERSION;

        let puell_multiple = RatioPerBlock::forced_import_raw(&db, "puell_multiple", v, indexes)?;
        let nvt = RatioPerBlock::forced_import_raw(&db, "nvt", v, indexes)?;
        let gini = PercentPerBlock::forced_import(&db, "gini", v, indexes)?;
        let rhodl_ratio = RatioPerBlock::forced_import_raw(&db, "rhodl_ratio", v, indexes)?;

        let this = Self {
            db,
            puell_multiple,
            nvt,
            gini,
            rhodl_ratio,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
