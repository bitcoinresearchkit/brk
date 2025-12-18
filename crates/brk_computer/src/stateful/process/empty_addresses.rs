use brk_error::Result;
use brk_types::{AnyAddressIndex, EmptyAddressData, EmptyAddressIndex, LoadedAddressIndex, OutputType, TypeIndex};

use super::EmptyAddressDataWithSource;
use crate::stateful::{AddressTypeToTypeIndexMap, AddressesDataVecs};

/// Process empty address data updates.
///
/// Handles three cases:
/// - New empty address: push to empty storage
/// - Updated empty address (was empty): update in place
/// - Transition loaded -> empty: delete from loaded, push to empty
///
/// Optimized to batch operations: deletes first (creates holes), then updates, then pushes.
pub fn process_empty_addresses(
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    // Estimate capacity from input size
    let total: usize = empty_updates.iter().map(|(_, m)| m.len()).sum();

    // Collect operations by type (no sorting needed)
    let mut updates: Vec<(EmptyAddressIndex, EmptyAddressData)> = Vec::with_capacity(total);
    let mut deletes: Vec<LoadedAddressIndex> = Vec::with_capacity(total);
    let mut pushes: Vec<(OutputType, TypeIndex, EmptyAddressData)> = Vec::with_capacity(total);

    for (address_type, items) in empty_updates.into_iter() {
        for (typeindex, source) in items {
            match source {
                EmptyAddressDataWithSource::New(data) => {
                    pushes.push((address_type, typeindex, data));
                }
                EmptyAddressDataWithSource::FromEmpty(index, data) => {
                    updates.push((index, data));
                }
                EmptyAddressDataWithSource::FromLoaded(loaded_index, data) => {
                    deletes.push(loaded_index);
                    pushes.push((address_type, typeindex, data));
                }
            }
        }
    }

    // Phase 1: All deletes (creates holes in loaded vec)
    for loaded_index in deletes {
        addresses_data.loaded.delete(loaded_index);
    }

    // Phase 2: All updates (in-place in empty vec)
    for (index, data) in updates {
        addresses_data.empty.update(index, data)?;
    }

    // Phase 3: All pushes (fills holes in empty vec, then grows)
    let mut result = AddressTypeToTypeIndexMap::default();
    for (address_type, typeindex, data) in pushes {
        let index = addresses_data.empty.fill_first_hole_or_push(data)?;
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(index));
    }

    Ok(result)
}
