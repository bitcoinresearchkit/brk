use brk_exit::Exit;
use vecdb::Database;

/// Shared control state for one complete plugin-composition update.
#[derive(Clone, Copy)]
pub struct UpdateContext<'a> {
    exit: &'a Exit,
}

impl<'a> UpdateContext<'a> {
    pub const fn new(exit: &'a Exit) -> Self {
        Self { exit }
    }

    pub const fn exit(self) -> &'a Exit {
        self.exit
    }

    /// Schedule deferred compaction under this update's shutdown lock.
    /// The next update must call `db.sync_bg_tasks()` before writing again.
    pub fn compact_database(self, db: &Database) {
        let exit = self.exit.clone();
        db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
    }
}

#[cfg(test)]
mod tests {
    use brk_types::{Height, StoredU64, Version};
    use vecdb::{AnyStoredVec, BytesVec, EagerVec, ImportableVec, ReadableVec, WritableVec};

    use super::*;

    #[test]
    fn compaction_completes_on_next_update_and_database_drop() {
        let directory = tempfile::tempdir().unwrap();
        let exit = Exit::new();
        let context = UpdateContext::new(&exit);
        let db = Database::open(directory.path()).unwrap();
        let mut values =
            EagerVec::<BytesVec<Height, StoredU64>>::forced_import(&db, "values", Version::ONE)
                .unwrap();

        for value in [11u64, 12] {
            values.push(value.into());
        }
        values.write().unwrap();
        context.compact_database(&db);

        // This is the barrier used at the beginning of the next plugin update.
        db.sync_bg_tasks().unwrap();
        for value in [21u64, 22] {
            values.push(value.into());
        }
        values.write().unwrap();
        context.compact_database(&db);
        drop(values);
        drop(db);

        // Dropping the last database owner must also join pending compaction.
        let db = Database::open(directory.path()).unwrap();
        let values =
            EagerVec::<BytesVec<Height, StoredU64>>::forced_import(&db, "values", Version::ONE)
                .unwrap();
        assert_eq!(values.collect(), [11u64, 12, 21, 22].map(StoredU64::from));
    }
}
