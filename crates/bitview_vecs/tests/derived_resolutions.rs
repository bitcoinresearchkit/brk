use bitview_traversable::Traversable;
use bitview_vecs::{DerivedResolutions, Resolutions};
use brk_types::{Height, StoredU64, Version};
use tempfile::tempdir;
use vecdb::{Database, Ident, ReadableCloneableVec, ReadableVec};

#[allow(dead_code)]
mod common;

#[test]
fn direct_transforms_preserve_sparse_epochs_and_export_metadata() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    indexes.first_height.day1 =
        common::stored(&db, "days", [0usize, 2, 2, 5, 20].map(Height::from))
            .read_only_boxed_clone();
    indexes.first_height.epoch =
        common::stored(&db, "epochs", [Height::ZERO]).read_only_boxed_clone();
    let source = common::stored::<Height, _>(&db, "values", (1..=8u64).map(StoredU64::from));
    let resolutions = Resolutions::from_source("values", &source, Version::ONE, &indexes);
    let transformed =
        DerivedResolutions::from_derived_computed::<Ident>("values", Version::ZERO, &resolutions);
    let chained = DerivedResolutions::from_lazy::<Ident, _>("values", Version::ZERO, &transformed);

    assert_eq!(transformed.day1.collect(), resolutions.day1.collect());
    assert!(transformed.day1.collect().contains(&None));
    assert_eq!(transformed.epoch.collect(), [StoredU64::from(8u64)]);
    for derived in [&transformed, &chained] {
        assert_eq!(derived.to_tree_node(), resolutions.to_tree_node());
        let actual: Vec<_> = derived.iter_any_exportable().collect();
        let expected: Vec<_> = resolutions.iter_any_exportable().collect();
        assert_eq!(actual.len(), 15);
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.into_iter().zip(expected) {
            assert_eq!(actual.name(), expected.name());
            assert_eq!(
                actual.index_type_to_string(),
                expected.index_type_to_string()
            );
            assert_eq!(actual.version(), expected.version());
            assert_eq!(actual.len(), expected.len());
            let mut actual_json = Vec::new();
            let mut expected_json = Vec::new();
            actual.write_json(None, None, &mut actual_json).unwrap();
            expected.write_json(None, None, &mut expected_json).unwrap();
            assert_eq!(actual_json, expected_json);
        }
    }
}
