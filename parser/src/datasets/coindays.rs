use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::AnyDataset,
    structs::{Config, MapKind},
    DateMap, HeightMap,
};

use super::{InsertData, MinInitialStates};

#[derive(Allocative, Iterable)]
pub struct CoindaysDataset {
    min_initial_states: MinInitialStates,
    pub coindays_destroyed: HeightMap<f32>,
    pub coindays_destroyed_1d_sum: DateMap<f32>,
}

impl CoindaysDataset {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Inserted
            coindays_destroyed: HeightMap::new_bin(1, MapKind::Inserted, &f("coindays_destroyed")),
            coindays_destroyed_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("coindays_destroyed_1d_sum"),
            ),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            satdays_destroyed,
            date_blocks_range,
            is_date_last_block,
            date,
            ..
        }: &InsertData,
    ) {
        self.coindays_destroyed
            .insert(height, satdays_destroyed.to_btc() as f32);

        if is_date_last_block {
            self.coindays_destroyed_1d_sum
                .insert(date, self.coindays_destroyed.sum_range(date_blocks_range));
        }
    }
}

impl AnyDataset for CoindaysDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
