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
    pub empty_output_index: EmptyOutputIndex,
    pub height: Height,
    pub op_return_index: OpReturnIndex,
    pub p2ms_output_index: P2MSOutputIndex,
    pub p2pk33_address_index: P2PK33AddressIndex,
    pub p2pk65_address_index: P2PK65AddressIndex,
    pub p2pkh_address_index: P2PKHAddressIndex,
    pub p2sh_address_index: P2SHAddressIndex,
    pub p2tr_address_index: P2TRAddressIndex,
    pub p2wpkh_address_index: P2WPKHAddressIndex,
    pub p2wsh_address_index: P2WSHAddressIndex,
    pub p2a_address_index: P2AAddressIndex,
    pub tx_index: TxIndex,
    pub txin_index: TxInIndex,
    pub txout_index: TxOutIndex,
    pub unknown_output_index: UnknownOutputIndex,
}

impl Indexes {
    pub fn to_type_index(&self, output_type: OutputType) -> TypeIndex {
        match output_type {
            OutputType::Empty => *self.empty_output_index,
            OutputType::OpReturn => *self.op_return_index,
            OutputType::P2A => *self.p2a_address_index,
            OutputType::P2MS => *self.p2ms_output_index,
            OutputType::P2PK33 => *self.p2pk33_address_index,
            OutputType::P2PK65 => *self.p2pk65_address_index,
            OutputType::P2PKH => *self.p2pkh_address_index,
            OutputType::P2SH => *self.p2sh_address_index,
            OutputType::P2TR => *self.p2tr_address_index,
            OutputType::P2WPKH => *self.p2wpkh_address_index,
            OutputType::P2WSH => *self.p2wsh_address_index,
            OutputType::Unknown => *self.unknown_output_index,
        }
    }

    /// Increments the address index for the given address type and returns the previous value.
    /// Only call this for address types (P2PK65, P2PK33, P2PKH, P2SH, P2WPKH, P2WSH, P2TR, P2A).
    #[inline]
    pub fn increment_address_index(&mut self, address_type: OutputType) -> TypeIndex {
        match address_type {
            OutputType::P2PK65 => self.p2pk65_address_index.copy_then_increment(),
            OutputType::P2PK33 => self.p2pk33_address_index.copy_then_increment(),
            OutputType::P2PKH => self.p2pkh_address_index.copy_then_increment(),
            OutputType::P2SH => self.p2sh_address_index.copy_then_increment(),
            OutputType::P2WPKH => self.p2wpkh_address_index.copy_then_increment(),
            OutputType::P2WSH => self.p2wsh_address_index.copy_then_increment(),
            OutputType::P2TR => self.p2tr_address_index.copy_then_increment(),
            OutputType::P2A => self.p2a_address_index.copy_then_increment(),
            _ => unreachable!(),
        }
    }
}
