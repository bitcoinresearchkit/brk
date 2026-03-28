use std::path::Path;

use brk_error::Result;
use brk_types::Version;

use crate::{
    indexes,
    internal::{
        CachedWindowStarts,
        db_utils::{finalize_db, open_db},
    },
};

use super::{HashrateVecs, RewardsVecs, Vecs};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 1_000_000)?;
        let version = parent_version;

        let rewards = RewardsVecs::forced_import(&db, version, indexes, cached_starts)?;
        let hashrate = HashrateVecs::forced_import(&db, version, indexes)?;

        let this = Self {
            db,
            rewards,
            hashrate,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
