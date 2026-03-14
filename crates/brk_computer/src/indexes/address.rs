use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    Address, AddressBytes, EmptyOutputIndex, OpReturnIndex, P2AAddressIndex, P2ABytes,
    P2MSOutputIndex, P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes,
    P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex, P2TRBytes,
    P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, TxIndex, UnknownOutputIndex,
    Version,
};
use vecdb::{LazyVecFrom1, ReadableCloneableVec};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub p2pk33: P2PK33Vecs,
    pub p2pk65: P2PK65Vecs,
    pub p2pkh: P2PKHVecs,
    pub p2sh: P2SHVecs,
    pub p2tr: P2TRVecs,
    pub p2wpkh: P2WPKHVecs,
    pub p2wsh: P2WSHVecs,
    pub p2a: P2AVecs,
    pub p2ms: P2MSVecs,
    pub empty: EmptyVecs,
    pub unknown: UnknownVecs,
    pub op_return: OpReturnVecs,
}

#[derive(Clone, Traversable)]
pub struct P2PK33Vecs {
    pub identity:
        LazyVecFrom1<P2PK33AddressIndex, P2PK33AddressIndex, P2PK33AddressIndex, P2PK33Bytes>,
    pub address: LazyVecFrom1<P2PK33AddressIndex, Address, P2PK33AddressIndex, P2PK33Bytes>,
}

#[derive(Clone, Traversable)]
pub struct P2PK65Vecs {
    pub identity:
        LazyVecFrom1<P2PK65AddressIndex, P2PK65AddressIndex, P2PK65AddressIndex, P2PK65Bytes>,
    pub address: LazyVecFrom1<P2PK65AddressIndex, Address, P2PK65AddressIndex, P2PK65Bytes>,
}

#[derive(Clone, Traversable)]
pub struct P2PKHVecs {
    pub identity: LazyVecFrom1<P2PKHAddressIndex, P2PKHAddressIndex, P2PKHAddressIndex, P2PKHBytes>,
    pub address: LazyVecFrom1<P2PKHAddressIndex, Address, P2PKHAddressIndex, P2PKHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2SHVecs {
    pub identity: LazyVecFrom1<P2SHAddressIndex, P2SHAddressIndex, P2SHAddressIndex, P2SHBytes>,
    pub address: LazyVecFrom1<P2SHAddressIndex, Address, P2SHAddressIndex, P2SHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2TRVecs {
    pub identity: LazyVecFrom1<P2TRAddressIndex, P2TRAddressIndex, P2TRAddressIndex, P2TRBytes>,
    pub address: LazyVecFrom1<P2TRAddressIndex, Address, P2TRAddressIndex, P2TRBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2WPKHVecs {
    pub identity:
        LazyVecFrom1<P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHBytes>,
    pub address: LazyVecFrom1<P2WPKHAddressIndex, Address, P2WPKHAddressIndex, P2WPKHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2WSHVecs {
    pub identity: LazyVecFrom1<P2WSHAddressIndex, P2WSHAddressIndex, P2WSHAddressIndex, P2WSHBytes>,
    pub address: LazyVecFrom1<P2WSHAddressIndex, Address, P2WSHAddressIndex, P2WSHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2AVecs {
    pub identity: LazyVecFrom1<P2AAddressIndex, P2AAddressIndex, P2AAddressIndex, P2ABytes>,
    pub address: LazyVecFrom1<P2AAddressIndex, Address, P2AAddressIndex, P2ABytes>,
}

#[derive(Clone, Traversable)]
pub struct P2MSVecs {
    pub identity: LazyVecFrom1<P2MSOutputIndex, P2MSOutputIndex, P2MSOutputIndex, TxIndex>,
}

#[derive(Clone, Traversable)]
pub struct EmptyVecs {
    pub identity: LazyVecFrom1<EmptyOutputIndex, EmptyOutputIndex, EmptyOutputIndex, TxIndex>,
}

#[derive(Clone, Traversable)]
pub struct UnknownVecs {
    pub identity: LazyVecFrom1<UnknownOutputIndex, UnknownOutputIndex, UnknownOutputIndex, TxIndex>,
}

#[derive(Clone, Traversable)]
pub struct OpReturnVecs {
    pub identity: LazyVecFrom1<OpReturnIndex, OpReturnIndex, OpReturnIndex, TxIndex>,
}

impl Vecs {
    pub(crate) fn forced_import(version: Version, indexer: &Indexer) -> Self {
        Self {
            p2pk33: P2PK33Vecs {
                identity: LazyVecFrom1::init(
                    "p2pk33_address_index",
                    version,
                    indexer.vecs.addresses.p2pk33.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2pk33_address",
                    version,
                    indexer.vecs.addresses.p2pk33.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2pk65: P2PK65Vecs {
                identity: LazyVecFrom1::init(
                    "p2pk65_address_index",
                    version,
                    indexer.vecs.addresses.p2pk65.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2pk65_address",
                    version,
                    indexer.vecs.addresses.p2pk65.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2pkh: P2PKHVecs {
                identity: LazyVecFrom1::init(
                    "p2pkh_address_index",
                    version,
                    indexer.vecs.addresses.p2pkh.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2pkh_address",
                    version,
                    indexer.vecs.addresses.p2pkh.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2sh: P2SHVecs {
                identity: LazyVecFrom1::init(
                    "p2sh_address_index",
                    version,
                    indexer.vecs.addresses.p2sh.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2sh_address",
                    version,
                    indexer.vecs.addresses.p2sh.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2tr: P2TRVecs {
                identity: LazyVecFrom1::init(
                    "p2tr_address_index",
                    version,
                    indexer.vecs.addresses.p2tr.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2tr_address",
                    version,
                    indexer.vecs.addresses.p2tr.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2wpkh: P2WPKHVecs {
                identity: LazyVecFrom1::init(
                    "p2wpkh_address_index",
                    version,
                    indexer.vecs.addresses.p2wpkh.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2wpkh_address",
                    version,
                    indexer.vecs.addresses.p2wpkh.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2wsh: P2WSHVecs {
                identity: LazyVecFrom1::init(
                    "p2wsh_address_index",
                    version,
                    indexer.vecs.addresses.p2wsh.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2wsh_address",
                    version,
                    indexer.vecs.addresses.p2wsh.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2a: P2AVecs {
                identity: LazyVecFrom1::init(
                    "p2a_address_index",
                    version,
                    indexer.vecs.addresses.p2a.bytes.read_only_boxed_clone(),
                    |index, _| index,
                ),
                address: LazyVecFrom1::init(
                    "p2a_address",
                    version,
                    indexer.vecs.addresses.p2a.bytes.read_only_boxed_clone(),
                    |_, bytes| Address::try_from(&AddressBytes::from(bytes)).unwrap(),
                ),
            },
            p2ms: P2MSVecs {
                identity: LazyVecFrom1::init(
                    "p2ms_output_index",
                    version,
                    indexer.vecs.scripts.p2ms.to_tx_index.read_only_boxed_clone(),
                    |index, _| index,
                ),
            },
            empty: EmptyVecs {
                identity: LazyVecFrom1::init(
                    "empty_output_index",
                    version,
                    indexer
                        .vecs
                        .scripts
                        .empty.to_tx_index
                        .read_only_boxed_clone(),
                    |index, _| index,
                ),
            },
            unknown: UnknownVecs {
                identity: LazyVecFrom1::init(
                    "unknown_output_index",
                    version,
                    indexer
                        .vecs
                        .scripts
                        .unknown.to_tx_index
                        .read_only_boxed_clone(),
                    |index, _| index,
                ),
            },
            op_return: OpReturnVecs {
                identity: LazyVecFrom1::init(
                    "op_return_index",
                    version,
                    indexer
                        .vecs
                        .scripts
                        .op_return.to_tx_index
                        .read_only_boxed_clone(),
                    |index, _| index,
                ),
            },
        }
    }
}
