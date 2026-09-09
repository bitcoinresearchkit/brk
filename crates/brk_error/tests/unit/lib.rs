use std::fs::TryLockError;

use vecdb::RawDBError;

use super::*;

#[test]
fn direct_rawdb_lock_is_classified_as_a_lock_error() {
    let error = Error::RawDB(RawDBError::TryLock(TryLockError::WouldBlock));

    assert!(error.is_lock_error());
    assert!(!error.is_data_error());
}
