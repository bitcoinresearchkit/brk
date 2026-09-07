use super::*;
use brk_types::Index;

fn field(name: &str, kind: &str) -> PatternField {
    PatternField {
        name: name.into(),
        rust_type: kind.into(),
        json_type: kind.into(),
        indexes: BTreeSet::from([Index::Height]),
        type_param: Some("existing".into()),
    }
}

#[test]
fn generic_normalization_preserves_wrapper_and_branch_rules() {
    for (types, expected) in [
        (["Cents", "Cents"], Some((["T", "T"], "Cents"))),
        (
            ["Open<Cents>", "Close<Cents>"],
            Some((["Open<T>", "Close<T>"], "Cents")),
        ),
        (
            ["Cents", "Close<Cents>"],
            Some((["T", "Close<T>"], "Cents")),
        ),
        (
            ["Open<Foo<Cents>>", "Close<Foo<Cents>>"],
            Some((["Open<T>", "Close<T>"], "Foo<Cents>")),
        ),
        (["Cents>", "Cents"], Some((["T", "T"], "Cents"))),
        (["Cents", "Sats"], None),
        (["Open<Cents>", "Close<Sats>"], None),
        (["Open<Cents", "Close<Cents>"], None),
    ] {
        let mut branch = field("child", "ChildPattern");
        branch.indexes.clear();
        let fields = vec![
            field("first", types[0]),
            branch.clone(),
            field("second", types[1]),
        ];
        let actual = normalize_fields_for_generic(&fields);
        match (actual, expected) {
            (None, None) => {}
            (Some((normalized, parameter)), Some((kinds, expected_parameter))) => {
                assert_eq!(parameter, expected_parameter);
                assert_eq!(format!("{:?}", normalized[1]), format!("{branch:?}"));
                for (index, kind) in [(0, kinds[0]), (2, kinds[1])] {
                    assert_eq!(normalized[index].rust_type, kind);
                    assert_eq!(normalized[index].json_type, kind);
                    assert_eq!(normalized[index].indexes, fields[index].indexes);
                    assert!(normalized[index].type_param.is_none());
                }
            }
            other => panic!("types={types:?}, result={other:?}"),
        }
    }
    assert!(normalize_fields_for_generic(&[]).is_none());
    let mut branch = field("child", "ChildPattern");
    branch.indexes.clear();
    assert!(normalize_fields_for_generic(&[branch]).is_none());
}

#[test]
fn borrowed_groups_keep_first_name_and_owned_mapping_contents() {
    let signatures = BTreeMap::from([
        (vec![field("close", "Cents")], "FirstPattern".into()),
        (vec![field("close", "Sats")], "SecondPattern".into()),
        (
            vec![field("unmatched", "Height")],
            "UnmatchedPattern".into(),
        ),
    ]);
    let (patterns, mappings, types) = detect_generic_patterns(&signatures);
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].name, "FirstPattern");
    assert!(patterns[0].is_generic);
    assert_eq!(mappings.len(), 2);
    assert!(mappings.values().all(|name| name == "FirstPattern"));
    assert_eq!(
        types.values().cloned().collect::<Vec<_>>(),
        ["Cents", "Sats"]
    );
    assert_eq!(signatures.len(), 3);
}
