use rawdb::Database;

use super::EagerVec;
use crate::{ImportOptions, ImportableVec, Result, Version};

impl<V: ImportableVec> ImportableVec for EagerVec<V> {
    fn import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(V::import(db, name, version)?))
    }

    fn import_with(options: ImportOptions) -> Result<Self> {
        Ok(Self(V::import_with(options)?))
    }

    fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(V::forced_import(db, name, version)?))
    }

    fn forced_import_with(options: ImportOptions) -> Result<Self> {
        Ok(Self(V::forced_import_with(options)?))
    }
}
