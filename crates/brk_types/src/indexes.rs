use crate::{
    EmptyOutputIndex, Height, OpReturnIndex, OutputType, P2AAddressIndex, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
    UnknownOutputIndex,
};

/// Blockchain-level indexes tracking current positions for various entity types.
/// Used by brk_indexer during block processing.
#[derive(Debug, Default, Clone)]
pub struct Indexes {
    pub emptyoutputindex: EmptyOutputIndex,
    pub height: Height,
    pub opreturnindex: OpReturnIndex,
    pub p2msoutputindex: P2MSOutputIndex,
    pub p2pk33addressindex: P2PK33AddressIndex,
    pub p2pk65addressindex: P2PK65AddressIndex,
    pub p2pkhaddressindex: P2PKHAddressIndex,
    pub p2shaddressindex: P2SHAddressIndex,
    pub p2traddressindex: P2TRAddressIndex,
    pub p2wpkhaddressindex: P2WPKHAddressIndex,
    pub p2wshaddressindex: P2WSHAddressIndex,
    pub p2aaddressindex: P2AAddressIndex,
    pub txindex: TxIndex,
    pub txinindex: TxInIndex,
    pub txoutindex: TxOutIndex,
    pub unknownoutputindex: UnknownOutputIndex,
}

impl Indexes {
    pub fn to_typeindex(&self, outputtype: OutputType) -> TypeIndex {
        match outputtype {
            OutputType::Empty => *self.emptyoutputindex,
            OutputType::OpReturn => *self.opreturnindex,
            OutputType::P2A => *self.p2aaddressindex,
            OutputType::P2MS => *self.p2msoutputindex,
            OutputType::P2PK33 => *self.p2pk33addressindex,
            OutputType::P2PK65 => *self.p2pk65addressindex,
            OutputType::P2PKH => *self.p2pkhaddressindex,
            OutputType::P2SH => *self.p2shaddressindex,
            OutputType::P2TR => *self.p2traddressindex,
            OutputType::P2WPKH => *self.p2wpkhaddressindex,
            OutputType::P2WSH => *self.p2wshaddressindex,
            OutputType::Unknown => *self.unknownoutputindex,
        }
    }

    /// Increments the address index for the given address type and returns the previous value.
    /// Only call this for address types (P2PK65, P2PK33, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2A).
    #[inline]
    pub fn increment_address_index(&mut self, addresstype: OutputType) -> TypeIndex {
        match addresstype {
            OutputType::P2PK65 => self.p2pk65addressindex.copy_then_increment(),
            OutputType::P2PK33 => self.p2pk33addressindex.copy_then_increment(),
            OutputType::P2PKH => self.p2pkhaddressindex.copy_then_increment(),
            OutputType::P2SH => self.p2shaddressindex.copy_then_increment(),
            OutputType::P2WPKH => self.p2wpkhaddressindex.copy_then_increment(),
            OutputType::P2WSH => self.p2wshaddressindex.copy_then_increment(),
            OutputType::P2TR => self.p2traddressindex.copy_then_increment(),
            OutputType::P2A => self.p2aaddressindex.copy_then_increment(),
            _ => unreachable!(),
        }
    }
}
