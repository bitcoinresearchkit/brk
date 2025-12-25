//! Type definitions for block processing.

use bitcoin::{Transaction, TxIn, TxOut};
use brk_types::{
    AddressBytes, AddressHash, Height, OutPoint, OutputType, Sats, TxIndex, TxOutIndex, Txid,
    TxidPrefix, TypeIndex, Vin, Vout,
};

/// Input source for tracking where an input came from.
#[derive(Debug)]
pub enum InputSource<'a> {
    PreviousBlock {
        vin: Vin,
        value: Sats,
        height: Height,
        txindex: TxIndex,
        txoutindex: TxOutIndex,
        outpoint: OutPoint,
        outputtype: OutputType,
        typeindex: TypeIndex,
    },
    SameBlock {
        txindex: TxIndex,
        txin: &'a TxIn,
        vin: Vin,
        outpoint: OutPoint,
    },
}

/// Output info for same-block spends (output created and spent in the same block).
#[derive(Debug, Clone, Copy)]
pub struct SameBlockOutputInfo {
    pub outputtype: OutputType,
    pub typeindex: TypeIndex,
    pub value: Sats,
    pub txoutindex: TxOutIndex,
}

/// Processed output data from parallel output processing.
pub struct ProcessedOutput<'a> {
    pub txoutindex: TxOutIndex,
    pub txout: &'a TxOut,
    pub txindex: TxIndex,
    pub vout: Vout,
    pub outputtype: OutputType,
    pub address_info: Option<(AddressBytes, AddressHash)>,
    pub existing_typeindex: Option<TypeIndex>,
}

/// Computed transaction data from parallel TXID computation.
pub struct ComputedTx<'a> {
    pub txindex: TxIndex,
    pub tx: &'a Transaction,
    pub txid: Txid,
    pub txid_prefix: TxidPrefix,
    pub prev_txindex_opt: Option<TxIndex>,
}
