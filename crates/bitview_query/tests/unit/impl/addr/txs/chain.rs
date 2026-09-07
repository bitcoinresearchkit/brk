use super::*;

#[test]
fn activity_anchor_distinguishes_an_empty_page_from_a_block_page() {
    let hash: BlockHash = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        .parse()
        .unwrap();
    let empty = ResolvedAddrChainTxs {
        txindices: vec![],
        anchor_height: None,
        activity_anchor: hash,
    };
    assert_eq!(empty.block_hash(), None);
    assert_eq!(empty.activity_anchor(), hash);

    let page = ResolvedAddrChainTxs {
        txindices: vec![TxIndex::ZERO],
        anchor_height: Some(Height::ZERO),
        activity_anchor: hash,
    };
    assert_eq!(page.block_hash(), Some(hash));
    assert_eq!(page.activity_anchor(), hash);
}
