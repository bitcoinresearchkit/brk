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
                // Address is in cache. Need to determine if it's been processed
                // by process_received (added to a cohort) or just loaded this block.
                //
                // - If wrapper is New AND funded_txo_count == 0: hasn't received yet,
                //   was just created in process_outputs this block → New
                // - If wrapper is New AND funded_txo_count > 0: received in previous
                //   block but still in cache (no flush) → Loaded
                // - If wrapper is FromLoaded/FromEmpty: loaded from storage → use wrapper
                let source = match entry.get() {
                    WithAddressDataSource::New(data) => {
                        if data.funded_txo_count == 0 {
                            AddressSource::New
                        } else {
                            AddressSource::Loaded
                        }
                    }
                    WithAddressDataSource::FromLoaded(..) => AddressSource::Loaded,
                    WithAddressDataSource::FromEmpty(_, data) => {
                        if data.utxo_count() == 0 {
                            AddressSource::FromEmpty
                        } else {
                            AddressSource::Loaded
                        }
                    }
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
