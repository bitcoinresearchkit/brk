use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;

use crate::{
    indexes,
    internal::{
        WindowStartVec, Windows,
        db_utils::{finalize_db, open_db},
    },
};

use super::{
    CountVecs, FeesVecs, InputTypesVecs, OutputTypesVecs, SizeVecs, Vecs, VersionsVecs, VolumeVecs,
};

impl Vecs {
    pub(crate) fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let db = open_db(parent_path, super::DB_NAME, 10_000_000)?;
        let version = parent_version;

        let count = CountVecs::forced_import(&db, version, indexes, cached_starts)?;
        let size = SizeVecs::forced_import(&db, version, indexer, indexes)?;
        let fees = FeesVecs::forced_import(&db, version, indexes)?;
        let versions = VersionsVecs::forced_import(&db, version, indexes, cached_starts)?;
        let volume = VolumeVecs::forced_import(&db, version, indexes, cached_starts)?;
        let input_types = InputTypesVecs::forced_import(&db, version, indexes, cached_starts)?;
        let output_types = OutputTypesVecs::forced_import(&db, version, indexes, cached_starts)?;

        let this = Self {
            db,
            count,
            size,
            fees,
            versions,
            volume,
            input_types,
            output_types,
        };
        finalize_db(&this.db, &this)?;
        Ok(this)
    }
}
