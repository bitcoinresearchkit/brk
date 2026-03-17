use bitcoin::{Transaction, TxOut};
use brk_cohort::ByAddrType;
use brk_types::{
    AddrBytes, AddrHash, OutPoint, OutputType, TxIndex, TxOutIndex, Txid, TxidPrefix,
    TypeIndex, Vin, Vout,
};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub enum InputSource {
    PreviousBlock {
        vin: Vin,
        tx_index: TxIndex,
        outpoint: OutPoint,
        output_type: OutputType,
        type_index: TypeIndex,
    },
    SameBlock {
        tx_index: TxIndex,
        vin: Vin,
        outpoint: OutPoint,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct SameBlockOutputInfo {
    pub output_type: OutputType,
    pub type_index: TypeIndex,
}

pub struct ProcessedOutput<'a> {
    pub txout_index: TxOutIndex,
    pub txout: &'a TxOut,
    pub tx_index: TxIndex,
    pub vout: Vout,
    pub output_type: OutputType,
    pub addr_info: Option<(AddrBytes, AddrHash)>,
    pub existing_type_index: Option<TypeIndex>,
}

pub struct ComputedTx<'a> {
    pub tx_index: TxIndex,
    pub tx: &'a Transaction,
    pub txid: Txid,
    pub txid_prefix: TxidPrefix,
    pub prev_tx_index_opt: Option<TxIndex>,
    pub base_size: u32,
    pub total_size: u32,
}

/// Reusable buffers cleared and refilled each block to avoid allocation churn.
#[derive(Default)]
pub struct BlockBuffers {
    pub txid_prefix_map: FxHashMap<TxidPrefix, TxIndex>,
    pub same_block_spent: FxHashSet<OutPoint>,
    pub already_added_addrs: ByAddrType<FxHashMap<AddrHash, TypeIndex>>,
    pub same_block_output_info: FxHashMap<OutPoint, SameBlockOutputInfo>,
}
