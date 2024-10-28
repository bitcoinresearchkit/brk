use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::AnyDataset,
    structs::{Config, Date, HeightMap, MapKind, Timestamp},
};

use super::{InsertData, MinInitialStates};

#[derive(Allocative, Iterable)]
pub struct BlockMetadataDataset {
    min_initial_states: MinInitialStates,
    pub date: HeightMap<Date>,
    pub timestamp: HeightMap<Timestamp>,
}

impl BlockMetadataDataset {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),
            // Inserted
            date: HeightMap::new_bin(1, MapKind::Inserted, &f("date")),
            timestamp: HeightMap::new_bin(1, MapKind::Inserted, &f("timestamp")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height, timestamp, ..
        }: &InsertData,
    ) {
        self.timestamp.insert(height, timestamp);

        self.date.insert(height, timestamp.to_date());
    }
}

impl AnyDataset for BlockMetadataDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
