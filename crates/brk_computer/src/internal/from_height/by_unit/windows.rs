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
        Windows::try_from_fn(|suffix| {
            ByUnit::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })
    }
}
