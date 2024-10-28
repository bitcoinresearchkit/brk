use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::AnyDataset,
    structs::{Config, DateMap, Height, MapKind},
};

use super::{InsertData, MinInitialStates};

#[derive(Allocative, Iterable)]
pub struct DateMetadataDataset {
    min_initial_states: MinInitialStates,

    pub first_height: DateMap<Height>,
    pub last_height: DateMap<Height>,
}

impl DateMetadataDataset {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Inserted
            first_height: DateMap::new_bin(1, MapKind::Inserted, &f("first_height")),
            last_height: DateMap::new_bin(1, MapKind::Inserted, &f("last_height")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            date,
            date_first_height,
            height,
            ..
        }: &InsertData,
    ) {
        self.first_height.insert(date, date_first_height);

        self.last_height.insert(date, height);
    }
}

impl AnyDataset for DateMetadataDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
