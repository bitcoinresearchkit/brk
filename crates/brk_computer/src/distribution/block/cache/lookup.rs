use brk_types::{LoadedAddressData, OutputType, TypeIndex};

use crate::distribution::address::AddressTypeToTypeIndexMap;

use super::super::cohort::{
    EmptyAddressDataWithSource, LoadedAddressDataWithSource, WithAddressDataSource,
};

/// Tracking status of an address - determines cohort update strategy.
#[derive(Clone, Copy)]
pub enum TrackingStatus {
    /// Brand new address (never seen before)
    New,
    /// Already tracked in a cohort (has existing balance)
    Tracked,
    /// Was in empty cache, now rejoining a cohort
    WasEmpty,
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
    ) -> (&mut LoadedAddressDataWithSource, TrackingStatus) {
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
                //   block but still in cache (no flush) → Tracked
                // - If wrapper is FromLoaded: loaded from storage → Tracked
                // - If wrapper is FromEmpty AND utxo_count == 0: still empty → WasEmpty
                // - If wrapper is FromEmpty AND utxo_count > 0: already received → Tracked
                let status = match entry.get() {
                    WithAddressDataSource::New(data) => {
                        if data.funded_txo_count == 0 {
                            TrackingStatus::New
                        } else {
                            TrackingStatus::Tracked
                        }
                    }
                    WithAddressDataSource::FromLoaded(..) => TrackingStatus::Tracked,
                    WithAddressDataSource::FromEmpty(_, data) => {
                        if data.utxo_count() == 0 {
                            TrackingStatus::WasEmpty
                        } else {
                            TrackingStatus::Tracked
                        }
                    }
                };
                (entry.into_mut(), status)
            }
            Entry::Vacant(entry) => {
                if let Some(empty_data) =
                    self.empty.get_mut(output_type).unwrap().remove(&type_index)
                {
                    return (entry.insert(empty_data.into()), TrackingStatus::WasEmpty);
                }
                (
                    entry.insert(WithAddressDataSource::New(LoadedAddressData::default())),
                    TrackingStatus::New,
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
