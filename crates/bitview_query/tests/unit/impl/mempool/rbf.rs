use super::*;

#[test]
fn empty_rbf_is_identified_by_its_exact_json() {
    let resolved = ResolvedRbf {
        source: RbfForTx::default(),
        tip: BlockHash::default(),
    };
    let bytes = serde_json::to_vec(&RbfResponse::EMPTY).unwrap();
    assert!(matches!(
        resolved.identity(),
        Some(RepresentationId::Content(hash)) if hash == RepresentationId::content_hash(&bytes)
    ));
}
