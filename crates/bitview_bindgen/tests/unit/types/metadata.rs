use super::*;
use crate::PatternMode;

#[test]
fn parameterizability_preserves_unknown_leaf_and_nested_branch_rules() {
    let mut metadata = ClientMetadata::from_catalog(TreeNode::Branch(Default::default()));
    let mode = Some(PatternMode::Suffix {
        relatives: BTreeMap::new(),
    });
    let field = |kind: &str| PatternField {
        name: "child".into(),
        rust_type: kind.into(),
        json_type: "object".into(),
        indexes: BTreeSet::new(),
        type_param: None,
    };
    metadata.structural_patterns = vec![
        StructuralPattern {
            name: "Parent".into(),
            fields: vec![field("Child")],
            mode: mode.clone(),
            is_generic: false,
        },
        StructuralPattern {
            name: "Child".into(),
            fields: vec![field("Unknown")],
            mode: None,
            is_generic: false,
        },
    ];
    assert!(!metadata.is_parameterizable("Unknown"));
    assert!(!metadata.is_parameterizable("Parent"));
    metadata.structural_patterns[1].mode = mode;
    assert!(metadata.is_parameterizable("Parent"));
    metadata.structural_patterns[1].mode = None;
    metadata.structural_patterns[0].fields[0]
        .indexes
        .insert(Index::Height);
    assert!(metadata.is_parameterizable("Parent"));
    metadata.structural_patterns[0].mode = None;
    assert!(!metadata.is_parameterizable("Parent"));
}
