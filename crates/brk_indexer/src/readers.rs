use bitcoin::ScriptBuf;
use brk_types::{
    AddressBytes, OutputType, P2AAddressIndex, P2ABytes, P2PK33AddressIndex, P2PK33Bytes,
    P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes,
    P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes,
    TxIndex, TxOutIndex, Txid, TypeIndex,
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

impl AddressReaders {
    pub fn script_pubkey(&self, output_type: OutputType, type_index: TypeIndex) -> ScriptBuf {
        let idx = usize::from(type_index);
        let bytes: Option<AddressBytes> = match output_type {
            OutputType::P2PK65 => self.p2pk65.try_get(idx).map(Into::into),
            OutputType::P2PK33 => self.p2pk33.try_get(idx).map(Into::into),
            OutputType::P2PKH => self.p2pkh.try_get(idx).map(Into::into),
            OutputType::P2SH => self.p2sh.try_get(idx).map(Into::into),
            OutputType::P2WPKH => self.p2wpkh.try_get(idx).map(Into::into),
            OutputType::P2WSH => self.p2wsh.try_get(idx).map(Into::into),
            OutputType::P2TR => self.p2tr.try_get(idx).map(Into::into),
            OutputType::P2A => self.p2a.try_get(idx).map(Into::into),
            _ => None,
        };
        bytes.map(|b| b.to_script_pubkey()).unwrap_or_default()
    }
}

/// Readers for vectors that need to be accessed during block processing.
///
/// All fields use `VecReader` which caches the mmap base pointer for O(1)
/// random access without recomputing `region.start() + HEADER_OFFSET` per read.
pub struct Readers {
    pub txid: VecReader<TxIndex, Txid, BytesStrategy<Txid>>,
    pub tx_index_to_first_txout_index: VecReader<TxIndex, TxOutIndex, BytesStrategy<TxOutIndex>>,
    pub txout_index_to_output_type: VecReader<TxOutIndex, OutputType, BytesStrategy<OutputType>>,
    pub txout_index_to_type_index: VecReader<TxOutIndex, TypeIndex, BytesStrategy<TypeIndex>>,
    pub addressbytes: AddressReaders,
}

impl Readers {
    pub fn new(vecs: &Vecs) -> Self {
        Self {
            txid: vecs.transactions.txid.reader(),
            tx_index_to_first_txout_index: vecs.transactions.first_txout_index.reader(),
            txout_index_to_output_type: vecs.outputs.output_type.reader(),
            txout_index_to_type_index: vecs.outputs.type_index.reader(),
            addressbytes: vecs.addresses.address_readers(),
        }
    }
}
