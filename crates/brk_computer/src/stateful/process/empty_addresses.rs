use brk_error::Result;
use brk_types::AnyAddressIndex;

use super::EmptyAddressDataWithSource;
use crate::stateful::{AddressTypeToTypeIndexMap, AddressesDataVecs};

/// Process empty address data updates.
///
/// Handles three cases:
/// - New empty address: push to empty storage
/// - Updated empty address (was empty): update in place
/// - Transition loaded -> empty: delete from loaded, push to empty
pub fn process_empty_addresses(
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let mut result = AddressTypeToTypeIndexMap::default();

    for (address_type, sorted) in empty_updates.into_sorted_iter() {
        for (typeindex, source) in sorted {
            match source {
                EmptyAddressDataWithSource::New(data) => {
                    let index = addresses_data.empty.fill_first_hole_or_push(data)?;
                    result
                        .get_mut(address_type)
                        .unwrap()
                        .insert(typeindex, AnyAddressIndex::from(index));
                }
                EmptyAddressDataWithSource::FromEmpty(index, data) => {
                    addresses_data.empty.update(index, data)?;
                }
                EmptyAddressDataWithSource::FromLoaded(loaded_index, data) => {
                    addresses_data.loaded.delete(loaded_index);
                    let empty_index = addresses_data.empty.fill_first_hole_or_push(data)?;
                    result
                        .get_mut(address_type)
                        .unwrap()
                        .insert(typeindex, AnyAddressIndex::from(empty_index));
                }
            }
        }
    }

    Ok(result)
}
