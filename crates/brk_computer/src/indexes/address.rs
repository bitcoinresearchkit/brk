use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{
    EmptyOutputIndex, OpReturnIndex, P2AAddressIndex, P2ABytes, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex,
    P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex,
    P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, TxIndex, UnknownOutputIndex, Version,
};
use vecdb::{IterableCloneableVec, LazyVecFrom1};

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
    pub opreturn: OpReturnVecs,
}

#[derive(Clone, Traversable)]
pub struct P2PK33Vecs {
    pub identity: LazyVecFrom1<P2PK33AddressIndex, P2PK33AddressIndex, P2PK33AddressIndex, P2PK33Bytes>,
}

#[derive(Clone, Traversable)]
pub struct P2PK65Vecs {
    pub identity: LazyVecFrom1<P2PK65AddressIndex, P2PK65AddressIndex, P2PK65AddressIndex, P2PK65Bytes>,
}

#[derive(Clone, Traversable)]
pub struct P2PKHVecs {
    pub identity: LazyVecFrom1<P2PKHAddressIndex, P2PKHAddressIndex, P2PKHAddressIndex, P2PKHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2SHVecs {
    pub identity: LazyVecFrom1<P2SHAddressIndex, P2SHAddressIndex, P2SHAddressIndex, P2SHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2TRVecs {
    pub identity: LazyVecFrom1<P2TRAddressIndex, P2TRAddressIndex, P2TRAddressIndex, P2TRBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2WPKHVecs {
    pub identity: LazyVecFrom1<P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2WSHVecs {
    pub identity: LazyVecFrom1<P2WSHAddressIndex, P2WSHAddressIndex, P2WSHAddressIndex, P2WSHBytes>,
}

#[derive(Clone, Traversable)]
pub struct P2AVecs {
    pub identity: LazyVecFrom1<P2AAddressIndex, P2AAddressIndex, P2AAddressIndex, P2ABytes>,
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
    pub fn forced_import(version: Version, indexer: &Indexer) -> Self {
        Self {
            p2pk33: P2PK33Vecs {
                identity: LazyVecFrom1::init(
                    "p2pk33addressindex",
                    version,
                    indexer.vecs.addresses.p2pk33bytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2pk65: P2PK65Vecs {
                identity: LazyVecFrom1::init(
                    "p2pk65addressindex",
                    version,
                    indexer.vecs.addresses.p2pk65bytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2pkh: P2PKHVecs {
                identity: LazyVecFrom1::init(
                    "p2pkhaddressindex",
                    version,
                    indexer.vecs.addresses.p2pkhbytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2sh: P2SHVecs {
                identity: LazyVecFrom1::init(
                    "p2shaddressindex",
                    version,
                    indexer.vecs.addresses.p2shbytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2tr: P2TRVecs {
                identity: LazyVecFrom1::init(
                    "p2traddressindex",
                    version,
                    indexer.vecs.addresses.p2trbytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2wpkh: P2WPKHVecs {
                identity: LazyVecFrom1::init(
                    "p2wpkhaddressindex",
                    version,
                    indexer.vecs.addresses.p2wpkhbytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2wsh: P2WSHVecs {
                identity: LazyVecFrom1::init(
                    "p2wshaddressindex",
                    version,
                    indexer.vecs.addresses.p2wshbytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2a: P2AVecs {
                identity: LazyVecFrom1::init(
                    "p2aaddressindex",
                    version,
                    indexer.vecs.addresses.p2abytes.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            p2ms: P2MSVecs {
                identity: LazyVecFrom1::init(
                    "p2msoutputindex",
                    version,
                    indexer.vecs.scripts.p2ms_to_txindex.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            empty: EmptyVecs {
                identity: LazyVecFrom1::init(
                    "emptyoutputindex",
                    version,
                    indexer.vecs.scripts.empty_to_txindex.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            unknown: UnknownVecs {
                identity: LazyVecFrom1::init(
                    "unknownoutputindex",
                    version,
                    indexer.vecs.scripts.unknown_to_txindex.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
            opreturn: OpReturnVecs {
                identity: LazyVecFrom1::init(
                    "opreturnindex",
                    version,
                    indexer.vecs.scripts.opreturn_to_txindex.boxed_clone(),
                    |index, _| Some(index),
                ),
            },
        }
    }
}
