use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Budgeted, BytesVec, Database, ImportableVec, ReadableVec, Version, WritableVec,
};

#[test]
fn no_cache_is_the_default_and_zero_budget_disables_retention() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut plain = BytesVec::<usize, u64>::import(&db, "plain", Version::ONE).unwrap();
    plain.push(7);
    plain.write().unwrap();
    assert_eq!(plain.collect(), [7]);
    assert!(!plain.read_cached_into_at(0, 1, &mut Vec::new()));
    assert!(BytesVec::<usize, u64, Budgeted>::import(&db, "missing_budget", Version::ONE).is_err());
    let budget = Budgeted::init_global(0).unwrap();
    assert!(Budgeted::init_global(4096).is_err());
    assert_eq!(Budgeted::global().unwrap().limit(), 0);
    let mut disabled =
        BytesVec::<usize, u64, Budgeted>::import(&db, "disabled", Version::ONE).unwrap();
    disabled.push(8);
    disabled.write().unwrap();
    assert_eq!(disabled.collect(), [8]);
    assert!(!disabled.read_cached_into_at(0, 1, &mut Vec::new()));
    assert_eq!(budget.used(), 0);
}
