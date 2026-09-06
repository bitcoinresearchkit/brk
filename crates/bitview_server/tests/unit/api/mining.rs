use axum::{http::header::ETAG, response::Response};
use bitcoin::hashes::Hash;
use brk_types::{BlockHash, Dollars, Height};

use crate::extended::ResponseExtended;

use super::pool_blocks_params;

#[test]
fn pool_page_identity_covers_sparse_selection_and_captured_prices() {
    let anchor = Some(BlockHash::default());
    let heights = [Height::from(100u32), Height::from(80u32)];
    let prices = [Dollars::from(10.0), Dollars::from(20.0)];
    let tag = |anchor, heights: &[Height], prices: &[Dollars]| {
        Response::new_not_modified(&pool_blocks_params(anchor, heights, prices)).headers()[ETAG]
            .clone()
    };
    let expected = tag(anchor, &heights, &prices);
    assert_eq!(expected, tag(anchor, &heights, &prices));
    assert_ne!(expected, tag(anchor, &[heights[0], 70u32.into()], &prices));
    assert_ne!(
        expected,
        tag(anchor, &heights, &[prices[0], Dollars::from(20.01)])
    );
    assert_ne!(expected, tag(anchor, &heights, &[prices[1], prices[0]]));
    assert_ne!(expected, tag(anchor, &heights[..1], &prices[..1]));
    let mut changed = [0; 32];
    changed[31] = 1;
    assert_ne!(
        expected,
        tag(
            Some(bitcoin::BlockHash::from_byte_array(changed).into()),
            &heights,
            &prices
        )
    );
    assert_eq!(tag(None, &[], &[]), tag(None, &[], &[]));
    assert_ne!(expected, tag(None, &[], &[]));
}
