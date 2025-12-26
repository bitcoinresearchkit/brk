use bitcoin::{Transaction, TxIn, TxOut};
use brk_types::{
    AddressBytes, AddressHash, OutPoint, OutputType, TxIndex, TxOutIndex, Txid, TxidPrefix,
    TypeIndex, Vin, Vout,
};

#[derive(Debug)]
pub enum InputSource<'a> {
    PreviousBlock {
        vin: Vin,
        txindex: TxIndex,
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
}
