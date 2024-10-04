use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, InsertData, MinInitialStates},
    states::InputState,
    structs::{AnyBiMap, BiMap},
};

#[derive(Default, Allocative)]
pub struct InputSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub count: BiMap<u64>,
    pub volume: BiMap<f64>,
    // Computed
    // add inputs_per_second
}

impl InputSubDataset {
    pub fn import(parent_path: &str, name: &Option<String>) -> color_eyre::Result<Self> {
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
            volume: BiMap::new_bin(1, &f("input_volume")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

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

        self.volume.height.insert(height, state.volume().to_btc());

        if is_date_last_block {
            self.count.date.insert(date, count);

            self.volume.date_insert_sum_range(date, date_blocks_range);
        }
    }
}

impl AnyDataset for InputSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count, &self.volume]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.count, &mut self.volume]
    }
}
