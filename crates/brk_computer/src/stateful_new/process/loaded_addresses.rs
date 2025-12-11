use brk_error::Result;
use brk_types::{AddressDataSource, AnyAddressIndex, LoadedAddressData};

use crate::stateful_new::{AddressTypeToTypeIndexMap, AddressesDataVecs};

/// Process loaded address data updates.
///
/// Handles three cases:
/// - New loaded address: push to loaded storage
/// - Updated loaded address (was loaded): update in place
/// - Transition empty -> loaded: delete from empty, push to loaded
pub fn process_loaded_addresses(
    addresses_data: &mut AddressesDataVecs,
    loaded_updates: AddressTypeToTypeIndexMap<AddressDataSource<LoadedAddressData>>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let mut result = AddressTypeToTypeIndexMap::default();

    for (address_type, sorted) in loaded_updates.into_sorted_iter() {
        for (typeindex, source) in sorted {
            match source {
                AddressDataSource::New(data) => {
                    let index = addresses_data.loaded.fill_first_hole_or_push(data)?;
                    result
                        .get_mut(address_type)
                        .unwrap()
                        .insert(typeindex, AnyAddressIndex::from(index));
                }
                AddressDataSource::FromLoaded((index, data)) => {
                    addresses_data.loaded.update(index, data)?;
                }
                AddressDataSource::FromEmpty((empty_index, data)) => {
                    addresses_data.empty.delete(empty_index);
                    let loaded_index = addresses_data.loaded.fill_first_hole_or_push(data)?;
                    result
                        .get_mut(address_type)
                        .unwrap()
                        .insert(typeindex, AnyAddressIndex::from(loaded_index));
                }
            }
        }
    }

    Ok(result)
}
