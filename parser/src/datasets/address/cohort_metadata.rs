use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, InsertData, MinInitialStates},
    structs::{BiMap, Config, MapKind},
};

#[derive(Allocative, Iterable)]
pub struct AddressCohortMetadataDataset {
    min_initial_states: MinInitialStates,

    address_count: BiMap<f64>,
    // pub output: OutputSubDataset,
    // Sending addresses
    // Receiving addresses
    // Active addresses (Unique(Sending + Receiving))
}

impl AddressCohortMetadataDataset {
    pub fn import(
        parent_path: &str,
        name: &Option<String>,
        config: &Config,
    ) -> color_eyre::Result<Self> {
        let f = |s: &str| {
            if let Some(name) = name {
                format!("{parent_path}/{name}/{s}")
            } else {
                format!("{parent_path}/{s}")
            }
        };

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Inserted
            address_count: BiMap::new_bin(1, MapKind::Inserted, &f("address_count")),
            // output: OutputSubDataset::import(parent_path)?,
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            date,
            is_date_last_block,
            ..
        }: &InsertData,
        address_count: f64,
    ) {
        self.address_count.height.insert(height, address_count);

        if is_date_last_block {
            self.address_count.date.insert(date, address_count);
        }
    }
}

impl AnyDataset for AddressCohortMetadataDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
