use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    parser::{
        datasets::{AnyDataset, InsertData, MinInitialStates},
        states::UTXOState,
    },
    structs::{BiMap, Config, MapKind, MapPath},
};

#[derive(Allocative, Iterable)]
pub struct UTXOSubDataset {
    min_initial_states: MinInitialStates,

    count: BiMap<f64>,
}

impl UTXOSubDataset {
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
            count: BiMap::new_bin(1, MapKind::Inserted, &f("utxo_count")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

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
}
