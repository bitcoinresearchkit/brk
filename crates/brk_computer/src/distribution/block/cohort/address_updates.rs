use brk_error::Result;
use brk_types::{
    AnyAddressIndex, EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex,
    OutputType, TypeIndex,
};
use vecdb::AnyVec;

use crate::distribution::{AddressTypeToTypeIndexMap, AddressesDataVecs};

use super::with_source::{EmptyAddressDataWithSource, FundedAddressDataWithSource};

/// Process funded address data updates.
///
/// Handles:
/// - New funded address: push to funded storage
/// - Updated funded address (was funded): update in place
/// - Transition empty -> funded: delete from empty, push to funded
pub(crate) fn process_funded_addresses(
    addresses_data: &mut AddressesDataVecs,
    funded_updates: AddressTypeToTypeIndexMap<FundedAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let total: usize = funded_updates.iter().map(|(_, m)| m.len()).sum();

    let mut updates: Vec<(FundedAddressIndex, FundedAddressData)> = Vec::with_capacity(total);
    let mut deletes: Vec<EmptyAddressIndex> = Vec::with_capacity(total);
    let mut pushes: Vec<(OutputType, TypeIndex, FundedAddressData)> = Vec::with_capacity(total);

    for (address_type, items) in funded_updates.into_iter() {
        for (typeindex, source) in items {
            match source {
                FundedAddressDataWithSource::New(data) => {
                    pushes.push((address_type, typeindex, data));
                }
                FundedAddressDataWithSource::FromFunded(index, data) => {
                    updates.push((index, data));
                }
                FundedAddressDataWithSource::FromEmpty(empty_index, data) => {
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
        addresses_data.funded.update(index, data)?;
    }

    // Phase 3: Pushes (fill holes first, then pure pushes)
    let mut result = AddressTypeToTypeIndexMap::with_capacity(pushes.len() / 4);
    let holes_count = addresses_data.funded.holes().len();
    let mut pushes_iter = pushes.into_iter();

    for (address_type, typeindex, data) in pushes_iter.by_ref().take(holes_count) {
        let index = addresses_data.funded.fill_first_hole_or_push(data)?;
        result
            .get_mut(address_type)
            .unwrap()
            .insert(typeindex, AnyAddressIndex::from(index));
    }

    // Pure pushes - no holes remain
    addresses_data.funded.reserve_pushed(pushes_iter.len());
    let mut next_index = addresses_data.funded.len();
    for (address_type, typeindex, data) in pushes_iter {
        addresses_data.funded.push(data);
        result.get_mut(address_type).unwrap().insert(
            typeindex,
            AnyAddressIndex::from(FundedAddressIndex::from(next_index)),
        );
        next_index += 1;
    }

    Ok(result)
}

/// Process empty address data updates.
///
/// Handles:
/// - New empty address: push to empty storage
/// - Updated empty address (was empty): update in place
/// - Transition funded -> empty: delete from funded, push to empty
pub(crate) fn process_empty_addresses(
    addresses_data: &mut AddressesDataVecs,
    empty_updates: AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
) -> Result<AddressTypeToTypeIndexMap<AnyAddressIndex>> {
    let total: usize = empty_updates.iter().map(|(_, m)| m.len()).sum();

    let mut updates: Vec<(EmptyAddressIndex, EmptyAddressData)> = Vec::with_capacity(total);
    let mut deletes: Vec<FundedAddressIndex> = Vec::with_capacity(total);
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
                EmptyAddressDataWithSource::FromFunded(funded_index, data) => {
                    deletes.push(funded_index);
                    pushes.push((address_type, typeindex, data));
                }
            }
        }
    }

    // Phase 1: Deletes (creates holes)
    for funded_index in deletes {
        addresses_data.funded.delete(funded_index);
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
        result.get_mut(address_type).unwrap().insert(
            typeindex,
            AnyAddressIndex::from(EmptyAddressIndex::from(next_index)),
        );
        next_index += 1;
    }

    Ok(result)
}
