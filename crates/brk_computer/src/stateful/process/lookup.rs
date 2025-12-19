//! Address data lookup during block processing.

use brk_types::{LoadedAddressData, OutputType, TypeIndex};

use super::super::address::AddressTypeToTypeIndexMap;
use super::{EmptyAddressDataWithSource, LoadedAddressDataWithSource, WithAddressDataSource};

/// Source of an address in lookup - reports where the data came from.
#[derive(Clone, Copy)]
pub enum AddressSource {
    /// Brand new address (never seen before)
    New,
    /// Loaded from disk (has existing balance)
    Loaded,
    /// Was empty (zero balance), now receiving
    FromEmpty,
}

/// Context for looking up and storing address data during block processing.
pub struct AddressLookup<'a> {
    pub loaded: &'a mut AddressTypeToTypeIndexMap<LoadedAddressDataWithSource>,
    pub empty: &'a mut AddressTypeToTypeIndexMap<EmptyAddressDataWithSource>,
}

impl<'a> AddressLookup<'a> {
    pub fn get_or_create_for_receive(
        &mut self,
        output_type: OutputType,
        type_index: TypeIndex,
    ) -> (&mut LoadedAddressDataWithSource, AddressSource) {
        use std::collections::hash_map::Entry;

        let map = self.loaded.get_mut(output_type).unwrap();

        match map.entry(type_index) {
            Entry::Occupied(entry) => {
                let source = match entry.get() {
                    WithAddressDataSource::New(_) => AddressSource::New,
                    WithAddressDataSource::FromLoaded(..) => AddressSource::Loaded,
                    WithAddressDataSource::FromEmpty(..) => AddressSource::FromEmpty,
                };
                (entry.into_mut(), source)
            }
            Entry::Vacant(entry) => {
                if let Some(empty_data) =
                    self.empty.get_mut(output_type).unwrap().remove(&type_index)
                {
                    return (entry.insert(empty_data.into()), AddressSource::FromEmpty);
                }
                (
                    entry.insert(WithAddressDataSource::New(LoadedAddressData::default())),
                    AddressSource::New,
                )
            }
        }
    }

    /// Get address data for a send operation (must exist in cache).
    pub fn get_for_send(
        &mut self,
        output_type: OutputType,
        type_index: TypeIndex,
    ) -> &mut LoadedAddressDataWithSource {
        self.loaded
            .get_mut(output_type)
            .unwrap()
            .get_mut(&type_index)
            .expect("Address must exist for send")
    }

    /// Move address from loaded to empty set.
    pub fn move_to_empty(&mut self, output_type: OutputType, type_index: TypeIndex) {
        let data = self
            .loaded
            .get_mut(output_type)
            .unwrap()
            .remove(&type_index)
            .unwrap();

        self.empty
            .get_mut(output_type)
            .unwrap()
            .insert(type_index, data.into());
    }
}
