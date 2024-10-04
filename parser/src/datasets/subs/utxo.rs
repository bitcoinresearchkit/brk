use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, InsertData, MinInitialStates},
    states::UTXOState,
    structs::{AnyBiMap, BiMap},
};

#[derive(Default, Allocative)]
pub struct UTXOSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    count: BiMap<f64>,
}

impl UTXOSubDataset {
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

            count: BiMap::new_bin(1, &f("utxo_count")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            is_date_last_block,
            date,
            ..
        }: &InsertData,
        state: &UTXOState,
    ) {
        let count = self.count.height.insert(height, state.count());

        if is_date_last_block {
            self.count.date.insert(date, count);
        }
    }
}

impl AnyDataset for UTXOSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.count]
    }
}
