use allocative::Allocative;

use crate::{
    datasets::AnyDataset,
    structs::{AnyDateMap, AnyHeightMap, Config},
    DateMap, HeightMap,
};

use super::{InsertData, MinInitialStates};

#[derive(Allocative)]
pub struct CoindaysDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub coindays_destroyed: HeightMap<f32>,
    pub coindays_destroyed_1d_sum: DateMap<f32>,
}

impl CoindaysDataset {
    pub fn import(parent_path: &str, config: &Config) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            coindays_destroyed: HeightMap::new_bin(1, &f("coindays_destroyed")),
            coindays_destroyed_1d_sum: DateMap::new_bin(1, &f("coindays_destroyed_1d_sum")),
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
    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.coindays_destroyed]
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.coindays_destroyed_1d_sum]
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![&mut self.coindays_destroyed]
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![&mut self.coindays_destroyed_1d_sum]
    }

    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
