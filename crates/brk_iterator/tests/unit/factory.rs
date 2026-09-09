use std::env;

use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::Height;

use crate::{Blocks, State};

fn offline_client() -> Client {
    Client::new("http://127.0.0.1:1", Auth::None).unwrap()
}

#[test]
fn empty_ranges_skip_both_sources_in_every_mode() {
    let client = offline_client();
    let reader = Reader::new_without_rlimit(
        env::temp_dir().join("brk-iterator-unused-block-directory"),
        &client,
    );
    for blocks in [
        Blocks::new_rpc(&client),
        Blocks::new_reader(&reader),
        Blocks::new(&client, &reader),
    ] {
        for mut iter in [
            blocks.last(0).unwrap(),
            blocks.range(Height::new(5), Height::new(4)).unwrap(),
            blocks.range(Height::new(u32::MAX), Height::ZERO).unwrap(),
        ] {
            assert!(matches!(iter.0, State::Empty));
            assert!(iter.next().is_none());
            assert!(iter.next().is_none());
        }
    }
}

#[test]
fn public_factory_preserves_the_full_inclusive_height_domain() {
    let blocks = Blocks::new_rpc(&offline_client());
    for iter in [
        blocks.range(Height::ZERO, Height::new(u32::MAX)).unwrap(),
        blocks.end(Height::new(u32::MAX)).unwrap(),
    ] {
        let State::Rpc { mut heights, .. } = iter.0 else {
            panic!("expected RPC")
        };
        assert_eq!(heights.next(), Some(0));
        assert_eq!(heights.next(), Some(1));
        assert_eq!(heights.next_back(), Some(u32::MAX));
        assert_eq!(heights.next_back(), Some(u32::MAX - 1));
    }
    for height in [0, u32::MAX] {
        let iter = blocks
            .range(Height::new(height), Height::new(height))
            .unwrap();
        let State::Rpc { mut heights, .. } = iter.0 else {
            panic!("expected RPC")
        };
        assert_eq!(heights.next(), Some(height));
        assert_eq!(heights.next(), None);
        assert_eq!(heights.next(), None);
    }
}
