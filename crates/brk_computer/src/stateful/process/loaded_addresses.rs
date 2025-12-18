use brk_error::Result;
use brk_types::{AnyAddressIndex, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex, OutputType, TypeIndex};

use super::LoadedAddressDataWithSource;
use crate::stateful::{AddressTypeToTypeIndexMap, AddressesDataVecs};

/// Process loaded address data updates.
///
/// Handles three cases:
/// - New loaded address: push to loaded storage
/// - Updated loaded address (was loaded): update in place
/// - Transition empty -> loaded: delete from empty, push to loaded
///
/// Optimized to batch operations: deletes first (creates holes), then updates, then pushes.
pub fn process_loaded_addresses(
    addresses_data: &mut AddressesDataVecs,
    loaded_updates: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    // Estimate capacity from input size
    let total: usize = loaded_updates.iter().map(|(_, m)| m.len()).sum();

    // Collect operations by type (no sorting needed)
    let mut updates: Vec<(LoadedAddressIndex, LoadedAddressData)> = Vec::with_capacity(total);
    let mut deletes: Vec<EmptyAddressIndex> = Vec::with_capacity(total);
    let mut pushes: Vec<(OutputType, TypeIndex, LoadedAddressData)> = Vec::with_capacity(total);

    for (address_type, items) in loaded_updates.into_iter() {
        for (typeindex, source) in items {
            match source {
                LoadedAddressDataWithSource::New(data) => {
                    pushes.push((address_type, typeindex, data));
                }
                LoadedAddressDataWithSource::FromLoaded(index, data) => {
                    updates.push((index, data));
                }
                LoadedAddressDataWithSource::FromEmpty(empty_index, data) => {
                    deletes.push(empty_index);
                    pushes.push((address_type, typeindex, data));
                }
            }
        }
    }

    // Phase 1: All deletes (creates holes in empty vec)
    for empty_index in deletes {
        addresses_data.empty.delete(empty_index);
    }

    // Phase 2: All updates (in-place in loaded vec)
    for (index, data) in updates {
        addresses_data.loaded.update(index, data)?;
    }

    // Phase 3: All pushes (fills holes in loaded vec, then grows)
    let mut result = AddressTypeToTypeIndexMap::default();
    for (address_type, typeindex, data) in pushes {
        let index = addresses_data.loaded.fill_first_hole_or_push(data)?;
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(index));
    }

    Ok(result)
}
