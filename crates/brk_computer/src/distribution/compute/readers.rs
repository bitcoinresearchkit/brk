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

/// Readers for txout vectors. Uses collect_range for bulk reads.
pub struct TxOutReaders<'a> {
    indexer: &'a Indexer,
}

impl<'a> TxOutReaders<'a> {
    pub(crate) fn new(indexer: &'a Indexer) -> Self {
        Self { indexer }
    }

    /// Collect output data for a block range using bulk reads.
    pub(crate) fn collect_block_outputs(
        &self,
        first_txoutindex: usize,
        output_count: usize,
    ) -> Vec<TxOutData> {
        let end = first_txoutindex + output_count;
        let values: Vec<Sats> = self.indexer.vecs.outputs.value.collect_range_at(first_txoutindex, end);
        let outputtypes: Vec<OutputType> = self.indexer.vecs.outputs.outputtype.collect_range_at(first_txoutindex, end);
        let typeindexes: Vec<TypeIndex> = self.indexer.vecs.outputs.typeindex.collect_range_at(first_txoutindex, end);

        values
            .into_iter()
            .zip(outputtypes)
            .zip(typeindexes)
            .map(|((value, outputtype), typeindex)| TxOutData {
                value,
                outputtype,
                typeindex,
            })
            .collect()
    }
}

/// Readers for txin vectors. Uses collect_range for bulk reads.
pub struct TxInReaders<'a> {
    indexer: &'a Indexer,
    txins: &'a inputs::Vecs,
    txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
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
        }
    }

    /// Collect input data for a block range using bulk reads.
    /// Computes prev_height on-the-fly from outpoint using RangeMap lookup.
    pub(crate) fn collect_block_inputs(
        &mut self,
        first_txinindex: usize,
        input_count: usize,
        current_height: Height,
    ) -> (Vec<Sats>, Vec<Height>, Vec<OutputType>, Vec<TypeIndex>) {
        let end = first_txinindex + input_count;
        let values: Vec<Sats> = self.txins.spent.value.collect_range_at(first_txinindex, end);
        let outpoints: Vec<OutPoint> = self.indexer.vecs.inputs.outpoint.collect_range_at(first_txinindex, end);
        let outputtypes: Vec<OutputType> = self.indexer.vecs.inputs.outputtype.collect_range_at(first_txinindex, end);
        let typeindexes: Vec<TypeIndex> = self.indexer.vecs.inputs.typeindex.collect_range_at(first_txinindex, end);

        let prev_heights: Vec<Height> = outpoints
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
