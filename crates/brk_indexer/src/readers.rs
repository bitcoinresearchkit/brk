use brk_types::{
    OutputType, P2AAddressIndex, P2ABytes, P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex,
    P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex,
    P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, TxIndex,
    TxOutIndex, Txid, TypeIndex,
};
use vecdb::{BytesStrategy, VecReader};

use crate::Vecs;

pub struct AddressReaders {
    pub p2pk65: VecReader<P2PK65AddressIndex, P2PK65Bytes, BytesStrategy<P2PK65Bytes>>,
    pub p2pk33: VecReader<P2PK33AddressIndex, P2PK33Bytes, BytesStrategy<P2PK33Bytes>>,
    pub p2pkh: VecReader<P2PKHAddressIndex, P2PKHBytes, BytesStrategy<P2PKHBytes>>,
    pub p2sh: VecReader<P2SHAddressIndex, P2SHBytes, BytesStrategy<P2SHBytes>>,
    pub p2wpkh: VecReader<P2WPKHAddressIndex, P2WPKHBytes, BytesStrategy<P2WPKHBytes>>,
    pub p2wsh: VecReader<P2WSHAddressIndex, P2WSHBytes, BytesStrategy<P2WSHBytes>>,
    pub p2tr: VecReader<P2TRAddressIndex, P2TRBytes, BytesStrategy<P2TRBytes>>,
    pub p2a: VecReader<P2AAddressIndex, P2ABytes, BytesStrategy<P2ABytes>>,
}

/// Readers for vectors that need to be accessed during block processing.
///
/// All fields use `VecReader` which caches the mmap base pointer for O(1)
/// random access without recomputing `region.start() + HEADER_OFFSET` per read.
pub struct Readers {
    pub txid: VecReader<TxIndex, Txid, BytesStrategy<Txid>>,
    pub txindex_to_first_txoutindex:
        VecReader<TxIndex, TxOutIndex, BytesStrategy<TxOutIndex>>,
    pub txoutindex_to_outputtype:
        VecReader<TxOutIndex, OutputType, BytesStrategy<OutputType>>,
    pub txoutindex_to_typeindex:
        VecReader<TxOutIndex, TypeIndex, BytesStrategy<TypeIndex>>,
    pub addressbytes: AddressReaders,
}

impl Readers {
    pub fn new(vecs: &Vecs) -> Self {
        Self {
            txid: vecs.transactions.txid.reader(),
            txindex_to_first_txoutindex: vecs.transactions.first_txoutindex.reader(),
            txoutindex_to_outputtype: vecs.outputs.outputtype.reader(),
            txoutindex_to_typeindex: vecs.outputs.typeindex.reader(),
            addressbytes: AddressReaders {
                p2pk65: vecs.addresses.p2pk65bytes.reader(),
                p2pk33: vecs.addresses.p2pk33bytes.reader(),
                p2pkh: vecs.addresses.p2pkhbytes.reader(),
                p2sh: vecs.addresses.p2shbytes.reader(),
                p2wpkh: vecs.addresses.p2wpkhbytes.reader(),
                p2wsh: vecs.addresses.p2wshbytes.reader(),
                p2tr: vecs.addresses.p2trbytes.reader(),
                p2a: vecs.addresses.p2abytes.reader(),
            },
        }
    }
}
