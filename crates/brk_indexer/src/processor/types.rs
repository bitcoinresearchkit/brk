use bitcoin::{Transaction, TxOut};
use brk_types::{
    AddressBytes, AddressHash, OutPoint, OutputType, TxIndex, TxOutIndex, Txid, TxidPrefix,
    TypeIndex, Vin, Vout,
};

#[derive(Debug)]
pub enum InputSource {
    PreviousBlock {
        vin: Vin,
        txindex: TxIndex,
        outpoint: OutPoint,
        outputtype: OutputType,
        typeindex: TypeIndex,
    },
    SameBlock {
        txindex: TxIndex,
        vin: Vin,
        outpoint: OutPoint,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct SameBlockOutputInfo {
    pub outputtype: OutputType,
    pub typeindex: TypeIndex,
}

pub struct ProcessedOutput<'a> {
    pub txoutindex: TxOutIndex,
    pub txout: &'a TxOut,
    pub txindex: TxIndex,
    pub vout: Vout,
    pub outputtype: OutputType,
    pub address_info: Option<(AddressBytes, AddressHash)>,
    pub existing_typeindex: Option<TypeIndex>,
}

pub struct ComputedTx<'a> {
    pub txindex: TxIndex,
    pub tx: &'a Transaction,
    pub txid: Txid,
    pub txid_prefix: TxidPrefix,
    pub prev_txindex_opt: Option<TxIndex>,
    pub base_size: u32,
    pub total_size: u32,
}
