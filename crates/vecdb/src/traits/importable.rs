use rawdb::Database;

use crate::{ImportOptions, Result, Version};

/// Trait for types that can be imported from a database.
///
/// This provides a uniform interface for constructing stored vectors,
/// enabling generic wrappers like `EagerVec` to work with any storage format.
pub trait ImportableVec: Sized {
    /// Import from database, creating if needed.
    fn import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Self::import_with((db, name, version).into())
    }

    /// Import with custom options.
    fn import_with(options: ImportOptions) -> Result<Self>;

    /// Import from database, resetting on version/format mismatch.
    fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Self::forced_import_with((db, name, version).into())
    }

    /// Forced import with custom options.
    fn forced_import_with(options: ImportOptions) -> Result<Self>;
}
