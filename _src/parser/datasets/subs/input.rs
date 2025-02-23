use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    parser::{
        datasets::{AnyDataset, InsertData, MinInitialStates},
        states::InputState,
    },
    structs::{BiMap, Config, DateMap, HeightMap, MapKind, MapPath},
};

#[derive(Allocative, Iterable)]
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
        path: &MapPath,
        name: &Option<String>,
        config: &Config,
    ) -> color_eyre::Result<Self> {
        let f = |s: &str| {
            if let Some(name) = name {
                path.join(&format!("{name}/{s}"))
            } else {
                path.join(s)
            }
        };

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // ---
            // Inserted
            // ---
            count: BiMap::new_bin(1, MapKind::Inserted, &f("input_count")),
            volume: HeightMap::new_bin(1, MapKind::Inserted, &f("input_volume")),
            volume_1d_sum: DateMap::new_bin(1, MapKind::Inserted, &f("input_volume_1d_sum")),
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
}
