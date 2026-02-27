use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use crate::{
    indexes,
    internal::{ByUnit, Windows},
};

impl Windows<ByUnit> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            _24h: ByUnit::forced_import(db, &format!("{name}_24h"), version, indexes)?,
            _7d: ByUnit::forced_import(db, &format!("{name}_7d"), version, indexes)?,
            _30d: ByUnit::forced_import(db, &format!("{name}_30d"), version, indexes)?,
            _1y: ByUnit::forced_import(db, &format!("{name}_1y"), version, indexes)?,
        })
    }
}
