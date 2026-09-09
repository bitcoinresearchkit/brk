use super::*;

#[test]
fn activity_anchor_is_available_for_empty_and_block_pages() {
    let hash: BlockHash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        .parse()
        .unwrap();
    let empty = ResolvedAddrChainTxs {
        txindices: vec![],
        anchor_height: None,
        activity_anchor: hash,
    };
    assert_eq!(empty.activity_anchor(), hash);

    let page = ResolvedAddrChainTxs {
        txindices: vec![TxIndex::ZERO],
        anchor_height: Some(Height::ZERO),
        activity_anchor: hash,
    };
    assert_eq!(page.activity_anchor(), hash);
}
