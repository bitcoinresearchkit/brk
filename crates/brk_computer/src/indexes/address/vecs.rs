use brk_traversable::Traversable;
use brk_types::{
    EmptyOutputIndex, OpReturnIndex, P2AAddressIndex, P2ABytes, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK33Bytes, P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex,
    P2PKHBytes, P2SHAddressIndex, P2SHBytes, P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex,
    P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes, TxIndex, UnknownOutputIndex,
};
use vecdb::LazyVecFrom1;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub emptyoutputindex_to_emptyoutputindex:
        LazyVecFrom1<EmptyOutputIndex, EmptyOutputIndex, EmptyOutputIndex, TxIndex>,
    pub opreturnindex_to_opreturnindex:
        LazyVecFrom1<OpReturnIndex, OpReturnIndex, OpReturnIndex, TxIndex>,
    pub p2aaddressindex_to_p2aaddressindex:
        LazyVecFrom1<P2AAddressIndex, P2AAddressIndex, P2AAddressIndex, P2ABytes>,
    pub p2msoutputindex_to_p2msoutputindex:
        LazyVecFrom1<P2MSOutputIndex, P2MSOutputIndex, P2MSOutputIndex, TxIndex>,
    pub p2pk33addressindex_to_p2pk33addressindex:
        LazyVecFrom1<P2PK33AddressIndex, P2PK33AddressIndex, P2PK33AddressIndex, P2PK33Bytes>,
    pub p2pk65addressindex_to_p2pk65addressindex:
        LazyVecFrom1<P2PK65AddressIndex, P2PK65AddressIndex, P2PK65AddressIndex, P2PK65Bytes>,
    pub p2pkhaddressindex_to_p2pkhaddressindex:
        LazyVecFrom1<P2PKHAddressIndex, P2PKHAddressIndex, P2PKHAddressIndex, P2PKHBytes>,
    pub p2shaddressindex_to_p2shaddressindex:
        LazyVecFrom1<P2SHAddressIndex, P2SHAddressIndex, P2SHAddressIndex, P2SHBytes>,
    pub p2traddressindex_to_p2traddressindex:
        LazyVecFrom1<P2TRAddressIndex, P2TRAddressIndex, P2TRAddressIndex, P2TRBytes>,
    pub p2wpkhaddressindex_to_p2wpkhaddressindex:
        LazyVecFrom1<P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHAddressIndex, P2WPKHBytes>,
    pub p2wshaddressindex_to_p2wshaddressindex:
        LazyVecFrom1<P2WSHAddressIndex, P2WSHAddressIndex, P2WSHAddressIndex, P2WSHBytes>,
    pub unknownoutputindex_to_unknownoutputindex:
        LazyVecFrom1<UnknownOutputIndex, UnknownOutputIndex, UnknownOutputIndex, TxIndex>,
}
