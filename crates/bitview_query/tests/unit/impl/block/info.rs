use std::io;

use bitcoin::{
    Network,
    blockdata::constants::genesis_block,
    consensus::{encode::VarInt, serialize},
};

use super::{BlockHash, Error, Query, TxIndex};

#[test]
fn block_ranges_use_exclusive_safe_bounds() {
    for len in 0..=8u32 {
        for start in [None, Some(0), Some(3), Some(8), Some(u32::MAX)] {
            for count in [0, 1, 3, 8, u32::MAX] {
                let (begin, end) =
                    Query::resolve_block_range(start.map(Into::into), count, len.into());
                let expected: Vec<_> = (0..len)
                    .rev()
                    .filter(|height| start.is_none_or(|start| *height <= start))
                    .take(count as usize)
                    .map(|height| height as usize)
                    .collect();
                assert_eq!((begin..end).rev().collect::<Vec<_>>(), expected);
                assert!(begin <= end && end <= len as usize);
                // Even a zero-row snapshot keeps the selected tip as its anchor.
                assert_eq!(
                    end,
                    start.map_or(len as usize, |height| {
                        (height as usize).saturating_add(1).min(len as usize)
                    })
                );
            }
        }
    }
    assert_eq!(
        Query::resolve_block_range(Some(u32::MAX.into()), u32::MAX, u32::MAX.into()),
        (0, u32::MAX as usize),
    );
}

#[test]
fn header_fields_must_belong_to_the_indexed_hash() {
    let header = genesis_block(Network::Bitcoin).header;
    let hash = BlockHash::from(header.block_hash());
    let bytes = serialize(&header);
    let decoded = Query::decode_header(&bytes, &hash).unwrap();
    assert_eq!(decoded.nonce, header.nonce);
    assert_eq!(decoded.bits, header.bits.to_consensus());
    assert_eq!(decoded.merkle_root, header.merkle_root.to_string());
    for end in 0..bytes.len() {
        assert!(Query::decode_header(&bytes[..end], &hash).is_err());
    }
    let mut extended = bytes.clone();
    extended.push(0);
    assert!(Query::decode_header(&extended, &hash).is_err());
    assert!(matches!(
        Query::decode_header(&bytes, &BlockHash::default()),
        Err(Error::Internal("Block header differs from index"))
    ));
    for index in 0..bytes.len() {
        let mut changed = bytes.clone();
        changed[index] ^= 1;
        assert!(
            matches!(
                Query::decode_header(&changed, &hash),
                Err(Error::Internal("Block header differs from index"))
            ),
            "byte {index}"
        );
    }
}

#[test]
fn stored_transaction_count_must_be_canonical_and_match_the_index() {
    for count in [1, 0xfc, 0xfd, 0xffff, 0x10000, u32::MAX] {
        let mut bytes = serialize(&VarInt(u64::from(count)));
        let encoded_len = bytes.len();
        for end in 0..encoded_len {
            assert!(Query::read_block_tx_count(&bytes[..end], count).is_err());
        }
        bytes.push(0xa5);
        let mut reader = bytes.as_slice();
        assert_eq!(
            Query::read_block_tx_count(&mut reader, count).unwrap(),
            encoded_len
        );
        assert_eq!(reader, &[0xa5], "count decoding consumed coinbase bytes");
    }
    for (bytes, expected) in [
        (vec![2], 1),
        (vec![0], 1),
        (vec![0xfd, 1, 0], 1),
        (vec![0xfe, 0xfd, 0, 0, 0], 0xfd),
        (serialize(&VarInt(u64::from(u32::MAX) + 1)), u32::MAX),
    ] {
        assert!(Query::read_block_tx_count(bytes.as_slice(), expected).is_err());
    }
}

#[test]
fn coinbase_read_failures_cannot_fabricate_extras() {
    let tx = genesis_block(Network::Bitcoin).txdata.remove(0);
    let bytes = serialize(&tx);
    let coinbase = Query::parse_coinbase_from_read(bytes.as_slice()).unwrap();
    assert_eq!(coinbase.scriptsig_bytes, tx.input[0].script_sig.as_bytes());
    assert_eq!(coinbase.total_size, bytes.len());
    assert_eq!(
        coinbase.payout_asm,
        tx.output[0].script_pubkey.to_asm_string()
    );
    for end in 0..bytes.len() {
        assert!(matches!(
            Query::parse_coinbase_from_read(&bytes[..end]),
            Err(Error::Internal(_))
        ));
    }
    struct Unreadable;
    impl io::Read for Unreadable {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("fixture read failure"))
        }
    }
    assert!(matches!(
        Query::parse_coinbase_from_read(Unreadable),
        Err(Error::Internal(_))
    ));
}

#[test]
fn transaction_counts_reject_invalid_published_boundaries() {
    for (first, next, limit, expected) in [
        (0, 1, 1, 1),
        (3, 7, 10, 4),
        (u32::MAX - 2, u32::MAX - 1, u32::MAX, 1),
    ] {
        assert_eq!(
            Query::block_tx_count(TxIndex::new(first), TxIndex::new(next), TxIndex::new(limit))
                .unwrap(),
            expected
        );
    }
    for (first, next, limit) in [(2, 2, 5), (3, 2, 5), (2, 6, 5), (8, 9, 5)] {
        assert!(matches!(
            Query::block_tx_count(TxIndex::new(first), TxIndex::new(next), TxIndex::new(limit)),
            Err(Error::Internal(_))
        ));
    }
}
