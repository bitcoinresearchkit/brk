use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, InsertData, MinInitialStates},
    states::InputState,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap, Config},
    DateMap, HeightMap,
};

#[derive(Default, Allocative)]
pub struct InputSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub count: BiMap<u64>,
    pub volume: HeightMap<f64>,
    pub volume_1d_sum: DateMap<f64>,
    // Computed
    // add inputs_per_second
}

impl InputSubDataset {
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

            count: BiMap::new_bin(1, &f("input_count")),
            volume: HeightMap::new_bin(1, &f("input_volume")),
            volume_1d_sum: DateMap::new_bin(1, &f("input_volume_1d_sum")),
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
            date_blocks_range,
            ..
        }: &InsertData,
        state: &InputState,
    ) {
        let count = self
            .count
            .height
            .insert(height, state.count().round() as u64);

        self.volume.insert(height, state.volume().to_btc());

        if is_date_last_block {
            self.count.date.insert(date, count);

            self.volume_1d_sum
                .insert(date, self.volume.sum_range(date_blocks_range));
        }
    }
}

impl AnyDataset for InputSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.count]
    }

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.volume]
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![&mut self.volume]
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![&self.volume_1d_sum]
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![&mut self.volume_1d_sum]
    }
}
