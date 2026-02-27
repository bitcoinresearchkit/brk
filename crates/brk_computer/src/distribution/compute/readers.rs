use brk_cohort::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{
    Height, OutPoint, OutputType, Sats, StoredU64, TxIndex, TypeIndex,
};
use vecdb::{Reader, ReadableVec, VecIndex};

use crate::{
    distribution::{
        RangeMap,
        address::{AddressesDataVecs, AnyAddressIndexesVecs},
    },
    inputs,
};

/// Output data collected from separate vecs.
#[derive(Debug, Clone, Copy)]
pub struct TxOutData {
    pub value: Sats,
    pub outputtype: OutputType,
    pub typeindex: TypeIndex,
}

/// Readers for txout vectors. Reuses internal buffers across blocks.
pub struct TxOutReaders<'a> {
    indexer: &'a Indexer,
    values_buf: Vec<Sats>,
    outputtypes_buf: Vec<OutputType>,
    typeindexes_buf: Vec<TypeIndex>,
}

impl<'a> TxOutReaders<'a> {
    pub(crate) fn new(indexer: &'a Indexer) -> Self {
        Self {
            indexer,
            values_buf: Vec::new(),
            outputtypes_buf: Vec::new(),
            typeindexes_buf: Vec::new(),
        }
    }

    /// Collect output data for a block range using bulk reads with buffer reuse.
    pub(crate) fn collect_block_outputs(
        &mut self,
        first_txoutindex: usize,
        output_count: usize,
    ) -> Vec<TxOutData> {
        let end = first_txoutindex + output_count;
        self.indexer.vecs.outputs.value.collect_range_into_at(first_txoutindex, end, &mut self.values_buf);
        self.indexer.vecs.outputs.outputtype.collect_range_into_at(first_txoutindex, end, &mut self.outputtypes_buf);
        self.indexer.vecs.outputs.typeindex.collect_range_into_at(first_txoutindex, end, &mut self.typeindexes_buf);

        self.values_buf
            .iter()
            .zip(&self.outputtypes_buf)
            .zip(&self.typeindexes_buf)
            .map(|((&value, &outputtype), &typeindex)| TxOutData {
                value,
                outputtype,
                typeindex,
            })
            .collect()
    }
}

/// Readers for txin vectors. Reuses outpoint buffer across blocks.
pub struct TxInReaders<'a> {
    indexer: &'a Indexer,
    txins: &'a inputs::Vecs,
    txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
    outpoints_buf: Vec<OutPoint>,
}

impl<'a> TxInReaders<'a> {
    pub(crate) fn new(
        indexer: &'a Indexer,
        txins: &'a inputs::Vecs,
        txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
    ) -> Self {
        Self {
            indexer,
            txins,
            txindex_to_height,
            outpoints_buf: Vec::new(),
        }
    }

    /// Collect input data for a block range using bulk reads.
    /// Outpoint buffer is reused across blocks; returned vecs are fresh (caller-owned).
    pub(crate) fn collect_block_inputs(
        &mut self,
        first_txinindex: usize,
        input_count: usize,
        current_height: Height,
    ) -> (Vec<Sats>, Vec<Height>, Vec<OutputType>, Vec<TypeIndex>) {
        let end = first_txinindex + input_count;
        let values: Vec<Sats> = self.txins.spent.value.collect_range_at(first_txinindex, end);
        self.indexer.vecs.inputs.outpoint.collect_range_into_at(first_txinindex, end, &mut self.outpoints_buf);
        let outputtypes: Vec<OutputType> = self.indexer.vecs.inputs.outputtype.collect_range_at(first_txinindex, end);
        let typeindexes: Vec<TypeIndex> = self.indexer.vecs.inputs.typeindex.collect_range_at(first_txinindex, end);

        let prev_heights: Vec<Height> = self.outpoints_buf
            .iter()
            .map(|outpoint| {
                if outpoint.is_coinbase() {
                    current_height
                } else {
                    self.txindex_to_height
                        .get(outpoint.txindex())
                        .unwrap_or(current_height)
                }
            })
            .collect();

        (values, prev_heights, outputtypes, typeindexes)
    }
}

/// Cached readers for stateful vectors.
pub struct VecsReaders {
    pub addresstypeindex_to_anyaddressindex: ByAddressType<Reader>,
    pub anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub(crate) fn new(
        any_address_indexes: &AnyAddressIndexesVecs,
        addresses_data: &AddressesDataVecs,
    ) -> Self {
        Self {
            addresstypeindex_to_anyaddressindex: ByAddressType {
                p2a: any_address_indexes.p2a.create_reader(),
                p2pk33: any_address_indexes.p2pk33.create_reader(),
                p2pk65: any_address_indexes.p2pk65.create_reader(),
                p2pkh: any_address_indexes.p2pkh.create_reader(),
                p2sh: any_address_indexes.p2sh.create_reader(),
                p2tr: any_address_indexes.p2tr.create_reader(),
                p2wpkh: any_address_indexes.p2wpkh.create_reader(),
                p2wsh: any_address_indexes.p2wsh.create_reader(),
            },
            anyaddressindex_to_anyaddressdata: ByAnyAddress {
                funded: addresses_data.funded.create_reader(),
                empty: addresses_data.empty.create_reader(),
            },
        }
    }

    /// Get reader for specific address type.
    pub(crate) fn address_reader(&self, address_type: OutputType) -> &Reader {
        self.addresstypeindex_to_anyaddressindex
            .get(address_type)
            .unwrap()
    }
}

/// Build txoutindex -> txindex mapping for a block.
pub(crate) fn build_txoutindex_to_txindex(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &impl ReadableVec<TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build txinindex -> txindex mapping for a block.
pub(crate) fn build_txinindex_to_txindex(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &impl ReadableVec<TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build index -> txindex mapping for a block (shared implementation).
fn build_index_to_txindex(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &impl ReadableVec<TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let first = block_first_txindex.to_usize();

    let counts: Vec<StoredU64> =
        txindex_to_count.collect_range_at(first, first + block_tx_count as usize);

    let total: u64 = counts.iter().map(|c| u64::from(*c)).sum();
    let mut result = Vec::with_capacity(total as usize);

    for (offset, count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(first + offset);
        result.extend(std::iter::repeat_n(txindex, u64::from(*count) as usize));
    }

    result
}
