use bitcoin::{Network, ScriptBuf, blockdata::constants::genesis_block, consensus::serialize};
use brk_rpc::{Auth, Client};
use tempfile::tempdir;

use super::*;

#[test]
fn stored_size_is_rejected_before_opening_the_block_file() {
    let directory = tempdir().unwrap();
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    let reader = Reader::new_without_rlimit(directory.path().join("missing"), &client);
    for size in [0, 4_000_001, usize::MAX] {
        assert!(matches!(
            read(&reader, BlkPosition::new(0, 0), size, Txid::COINBASE),
            Err(Error::Internal(_))
        ));
    }
}

#[test]
fn decoding_requires_exact_bytes_identity_and_weight() {
    let transaction = genesis_block(Network::Bitcoin).txdata.remove(0);
    let txid = transaction.compute_txid().into();
    let bytes = serialize(&transaction);
    assert_eq!(decode(&bytes, txid).unwrap(), transaction);
    assert!(decode(&bytes, Txid::COINBASE).is_err());
    assert!(decode(&bytes[..bytes.len() - 1], txid).is_err());
    let mut trailing = bytes;
    trailing.push(0);
    assert!(decode(&trailing, txid).is_err());
    let mut overweight = transaction;
    overweight.input[0].script_sig = ScriptBuf::from_bytes(vec![0; 1_000_001]);
    assert!(decode(&serialize(&overweight), overweight.compute_txid().into()).is_err());
}
