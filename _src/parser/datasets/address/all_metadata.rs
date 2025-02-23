use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    parser::datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    structs::{BiMap, Config, MapKind, MapPath},
};

#[derive(Allocative, Iterable)]
pub struct AllAddressesMetadataDataset {
    min_initial_states: MinInitialStates,
    created_addreses: BiMap<u32>,
    empty_addresses: BiMap<u32>,
    new_addresses: BiMap<u32>,
}

impl AllAddressesMetadataDataset {
    pub fn import(path: &MapPath, config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| path.join(s);

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Inserted
            created_addreses: BiMap::new_bin(1, MapKind::Inserted, &f("created_addresses")),
            empty_addresses: BiMap::new_bin(1, MapKind::Inserted, &f("empty_addresses")),

            // Computed
            new_addresses: BiMap::new_bin(1, MapKind::Computed, &f("new_addresses")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(&mut self, insert_data: &InsertData) {
        let &InsertData {
            databases,
            height,
            date,
            is_date_last_block,
            ..
        } = insert_data;

        let created_addresses = self
            .created_addreses
            .height
            .insert(height, *databases.address_to_address_index.metadata.len);

        let empty_addresses = self.empty_addresses.height.insert(
            height,
            *databases.address_index_to_empty_address_data.metadata.len,
        );

        if is_date_last_block {
            self.created_addreses.date.insert(date, created_addresses);

            self.empty_addresses.date.insert(date, empty_addresses);
        }
    }

    pub fn compute(&mut self, &ComputeData { heights, dates, .. }: &ComputeData) {
        self.new_addresses
            .multi_insert_net_change(heights, dates, &mut self.created_addreses, 1)
    }
}

impl AnyDataset for AllAddressesMetadataDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
