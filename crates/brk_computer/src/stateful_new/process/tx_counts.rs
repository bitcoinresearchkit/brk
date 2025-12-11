//! Transaction count tracking per address.
//!
//! Updates tx_count on address data after deduplicating transaction indexes.

use crate::stateful_new::address::AddressTypeToTypeIndexMap;

use super::{EmptyAddressDataWithSource, LoadedAddressDataWithSource, TxIndexVec};

/// Update tx_count for addresses based on unique transactions they participated in.
///
/// For each address:
/// 1. Deduplicate transaction indexes (an address may appear in multiple inputs/outputs of same tx)
/// 2. Add the unique count to the address's tx_count field
///
/// Addresses are looked up in loaded_cache first, then empty_cache.
/// NOTE: This should be called AFTER merging parallel-fetched address data into loaded_cache.
pub fn update_tx_counts(
    loaded_cache: &mut AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    empty_cache: &mut AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
    mut txindex_vecs: AddressTypeToTypeIndexMap<TxIndexVec>,
) {
    // First, deduplicate txindex_vecs for addresses that appear multiple times in a block
    for (_, map) in txindex_vecs.iter_mut() {
        for (_, txindex_vec) in map.iter_mut() {
            if txindex_vec.len() > 1 {
                txindex_vec.sort_unstable();
                txindex_vec.dedup();
            }
        }
    }

    // Update tx_count on address data
    for (address_type, typeindex, txindex_vec) in txindex_vecs
        .into_iter()
        .flat_map(|(t, m)| m.into_iter().map(move |(i, v)| (t, i, v)))
    {
        let tx_count = txindex_vec.len() as u32;

        if let Some(addr_data) = loaded_cache
            .get_mut(address_type)
            .unwrap()
            .get_mut(&typeindex)
        {
            addr_data.tx_count += tx_count;
        } else if let Some(addr_data) = empty_cache
            .get_mut(address_type)
            .unwrap()
            .get_mut(&typeindex)
        {
            addr_data.tx_count += tx_count;
        }
    }
}
