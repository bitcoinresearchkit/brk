use brk_cohort::{ByAddressType, ByAnyAddress};
use brk_indexer::Indexer;
use brk_types::{
    Height, OutPoint, OutputType, Sats, StoredU64, TxInIndex, TxIndex, TxOutIndex, TypeIndex,
};
use vecdb::{
    BoxedVecIterator, BytesVecIterator, GenericStoredVec, PcodecVecIterator, Reader, VecIndex,
    VecIterator,
};

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

/// Reusable iterators for txout vectors (16KB buffered reads).
///
/// Iterators are created once and re-positioned each block to avoid
/// creating new file handles repeatedly.
pub struct TxOutIterators<'a> {
    value_iter: BytesVecIterator<'a, TxOutIndex, Sats>,
    outputtype_iter: BytesVecIterator<'a, TxOutIndex, OutputType>,
    typeindex_iter: BytesVecIterator<'a, TxOutIndex, TypeIndex>,
}

impl<'a> TxOutIterators<'a> {
    pub fn new(indexer: &'a Indexer) -> Self {
        Self {
            value_iter: indexer.vecs.txout.txoutindex_to_value.into_iter(),
            outputtype_iter: indexer.vecs.txout.txoutindex_to_outputtype.into_iter(),
            typeindex_iter: indexer.vecs.txout.txoutindex_to_typeindex.into_iter(),
        }
    }

    /// Collect output data for a block range using buffered iteration.
    pub fn collect_block_outputs(
        &mut self,
        first_txoutindex: usize,
        output_count: usize,
    ) -> Vec<TxOutData> {
        (first_txoutindex..first_txoutindex + output_count)
            .map(|i| TxOutData {
                value: self.value_iter.get_at_unwrap(i),
                outputtype: self.outputtype_iter.get_at_unwrap(i),
                typeindex: self.typeindex_iter.get_at_unwrap(i),
            })
            .collect()
    }
}

/// Reusable iterators for txin vectors (PcoVec - avoids repeated page decompression).
pub struct TxInIterators<'a> {
    value_iter: PcodecVecIterator<'a, TxInIndex, Sats>,
    outpoint_iter: PcodecVecIterator<'a, TxInIndex, OutPoint>,
    outputtype_iter: PcodecVecIterator<'a, TxInIndex, OutputType>,
    typeindex_iter: PcodecVecIterator<'a, TxInIndex, TypeIndex>,
    txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
}

impl<'a> TxInIterators<'a> {
    pub fn new(
        indexer: &'a Indexer,
        txins: &'a inputs::Vecs,
        txindex_to_height: &'a mut RangeMap<TxIndex, Height>,
    ) -> Self {
        Self {
            value_iter: txins.spent.txinindex_to_value.into_iter(),
            outpoint_iter: indexer.vecs.txin.txinindex_to_outpoint.into_iter(),
            outputtype_iter: indexer.vecs.txin.txinindex_to_outputtype.into_iter(),
            typeindex_iter: indexer.vecs.txin.txinindex_to_typeindex.into_iter(),
            txindex_to_height,
        }
    }

    /// Collect input data for a block range using buffered iteration.
    /// Computes prev_height on-the-fly from outpoint using RangeMap lookup.
    pub fn collect_block_inputs(
        &mut self,
        first_txinindex: usize,
        input_count: usize,
        current_height: Height,
    ) -> (Vec<Sats>, Vec<Height>, Vec<OutputType>, Vec<TypeIndex>) {
        let mut values = Vec::with_capacity(input_count);
        let mut prev_heights = Vec::with_capacity(input_count);
        let mut outputtypes = Vec::with_capacity(input_count);
        let mut typeindexes = Vec::with_capacity(input_count);

        for i in first_txinindex..first_txinindex + input_count {
            values.push(self.value_iter.get_at_unwrap(i));

            let outpoint = self.outpoint_iter.get_at_unwrap(i);
            let prev_height = if outpoint.is_coinbase() {
                current_height
            } else {
                self.txindex_to_height
                    .get(outpoint.txindex())
                    .unwrap_or(current_height)
            };
            prev_heights.push(prev_height);

            outputtypes.push(self.outputtype_iter.get_at_unwrap(i));
            typeindexes.push(self.typeindex_iter.get_at_unwrap(i));
        }

        (values, prev_heights, outputtypes, typeindexes)
    }
}

/// Cached readers for stateful vectors.
pub struct VecsReaders {
    pub addresstypeindex_to_anyaddressindex: ByAddressType<Reader>,
    pub anyaddressindex_to_anyaddressdata: ByAnyAddress<Reader>,
}

impl VecsReaders {
    pub fn new(
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
                loaded: addresses_data.loaded.create_reader(),
                empty: addresses_data.empty.create_reader(),
            },
        }
    }

    /// Get reader for specific address type.
    pub fn address_reader(&self, address_type: OutputType) -> &Reader {
        self.addresstypeindex_to_anyaddressindex
            .get_unwrap(address_type)
    }
}

/// Build txoutindex -> txindex mapping for a block.
pub fn build_txoutindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build txinindex -> txindex mapping for a block.
pub fn build_txinindex_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    build_index_to_txindex(block_first_txindex, block_tx_count, txindex_to_count)
}

/// Build index -> txindex mapping for a block (shared implementation).
fn build_index_to_txindex<'a>(
    block_first_txindex: TxIndex,
    block_tx_count: u64,
    txindex_to_count: &mut BoxedVecIterator<'a, TxIndex, StoredU64>,
) -> Vec<TxIndex> {
    let first = block_first_txindex.to_usize();

    let counts: Vec<u64> = (0..block_tx_count as usize)
        .map(|offset| {
            let txindex = TxIndex::from(first + offset);
            u64::from(txindex_to_count.get_unwrap(txindex))
        })
        .collect();

    let total: u64 = counts.iter().sum();
    let mut result = Vec::with_capacity(total as usize);

    for (offset, &count) in counts.iter().enumerate() {
        let txindex = TxIndex::from(first + offset);
        result.extend(std::iter::repeat_n(txindex, count as usize));
    }

    result
}
