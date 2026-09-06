use super::*;

#[test]
fn direct_rawdb_lock_is_classified_as_a_lock_error() {
    let error = Error::RawDB(vecdb::RawDBError::TryLock(
        std::fs::TryLockError::WouldBlock,
    ));

    assert!(error.is_lock_error());
    assert!(!error.is_data_error());
}
