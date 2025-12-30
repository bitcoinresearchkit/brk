use brk_error::Result;
use brk_types::{
    AnyAddressIndex, EmptyAddressData, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex,
    OutputType, TypeIndex,
};

use super::{EmptyAddressDataWithSource, LoadedAddressDataWithSource};
use crate::stateful::{AddressTypeToTypeIndexMap, AddressesDataVecs};

/// Process loaded address data updates.
///
/// Handles:
/// - New loaded address: push to loaded storage
/// - Updated loaded address (was loaded): update in place
/// - Transition empty -> loaded: delete from empty, push to loaded
pub fn process_loaded_addresses(
    addresses_data: &mut AddressesDataVecs,
    loaded_updates: AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let total: usize = loaded_updates.iter().map(|(_, m)| m.len()).sum();

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

    // Phase 1: Deletes (creates holes)
    for empty_index in deletes {
        addresses_data.empty.delete(empty_index);
    }

    // Phase 2: Updates (in-place)
    for (index, data) in updates {
        addresses_data.loaded.update(index, data)?;
    }

    // Phase 3: Pushes (fills holes, then grows)
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

/// Process empty address data updates.
///
/// Handles:
/// - New empty address: push to empty storage
/// - Updated empty address (was empty): update in place
/// - Transition loaded -> empty: delete from loaded, push to empty
pub fn process_empty_addresses(
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let total: usize = empty_updates.iter().map(|(_, m)| m.len()).sum();

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

    // Phase 1: Deletes (creates holes)
    for loaded_index in deletes {
        addresses_data.loaded.delete(loaded_index);
    }

    // Phase 2: Updates (in-place)
    for (index, data) in updates {
        addresses_data.empty.update(index, data)?;
    }

    // Phase 3: Pushes (fills holes, then grows)
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
