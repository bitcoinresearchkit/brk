use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    structs::{AnyBiMap, BiMap},
};

#[derive(Allocative)]
pub struct AllAddressesMetadataDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    created_addreses: BiMap<u32>,
    empty_addresses: BiMap<u32>,

    // Computed
    new_addresses: BiMap<u32>,
}

impl AllAddressesMetadataDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            created_addreses: BiMap::new_bin(1, &f("created_addresses")),
            empty_addresses: BiMap::new_bin(1, &f("empty_addresses")),
            new_addresses: BiMap::new_bin(1, &f("new_addresses")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

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

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.created_addreses, &self.empty_addresses]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.created_addreses, &mut self.empty_addresses]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.new_addresses]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.new_addresses]
    }
}
