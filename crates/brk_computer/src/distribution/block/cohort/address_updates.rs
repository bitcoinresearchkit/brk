use brk_error::Result;
use brk_types::{
    AnyAddressIndex, EmptyAddressData, EmptyAddressIndex, LoadedAddressData, LoadedAddressIndex,
    OutputType, TypeIndex,
};
use vecdb::AnyVec;

use crate::distribution::{AddressTypeToTypeIndexMap, AddressesDataVecs};

use super::with_source::{EmptyAddressDataWithSource, LoadedAddressDataWithSource};

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

    // Phase 3: Pushes (fill holes first, then pure pushes)
    let mut result = AddressTypeToTypeIndexMap::with_capacity(pushes.len() / 4);
    let holes_count = addresses_data.loaded.holes().len();
    let mut pushes_iter = pushes.into_iter();

    for (address_type, typeindex, data) in pushes_iter.by_ref().take(holes_count) {
        let index = addresses_data.loaded.fill_first_hole_or_push(data)?;
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(index));
    }

    // Pure pushes - no holes remain
    addresses_data.loaded.reserve_pushed(pushes_iter.len());
    let mut next_index = addresses_data.loaded.len();
    for (address_type, typeindex, data) in pushes_iter {
        addresses_data.loaded.push(data);
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(LoadedAddressIndex::from(next_index)));
        next_index += 1;
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

    // Phase 3: Pushes (fill holes first, then pure pushes)
    let mut result = AddressTypeToTypeIndexMap::with_capacity(pushes.len() / 4);
    let holes_count = addresses_data.empty.holes().len();
    let mut pushes_iter = pushes.into_iter();

    for (address_type, typeindex, data) in pushes_iter.by_ref().take(holes_count) {
        let index = addresses_data.empty.fill_first_hole_or_push(data)?;
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(index));
    }

    // Pure pushes - no holes remain
    addresses_data.empty.reserve_pushed(pushes_iter.len());
    let mut next_index = addresses_data.empty.len();
    for (address_type, typeindex, data) in pushes_iter {
        addresses_data.empty.push(data);
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(EmptyAddressIndex::from(next_index)));
        next_index += 1;
    }

    Ok(result)
}
