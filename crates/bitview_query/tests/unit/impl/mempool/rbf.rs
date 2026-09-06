use super::*;
use crate::representation_id::content_hash;

#[test]
fn empty_rbf_is_identified_by_its_exact_json() {
    let resolved = ResolvedRbf::new(RbfForTx::default(), BlockHash::default());
    let bytes = serde_json::to_vec(&RbfResponse::EMPTY).unwrap();
    assert!(matches!(
        resolved.identity(),
        Some(RepresentationId::Content(hash)) if hash == content_hash(&bytes)
    ));
}
