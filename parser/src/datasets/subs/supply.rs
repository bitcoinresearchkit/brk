use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::SupplyState,
    structs::{BiMap, Config, MapKind},
};

#[derive(Allocative, Iterable)]
pub struct SupplySubDataset {
    min_initial_states: MinInitialStates,

    pub supply: BiMap<f64>,
    pub supply_to_circulating_supply_ratio: BiMap<f64>,
    pub halved_supply: BiMap<f64>,
    pub halved_supply_to_circulating_supply_ratio: BiMap<f64>,
}

impl SupplySubDataset {
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

            // ---
            // Inserted
            // ---
            supply: BiMap::new_bin(1, MapKind::Inserted, &f("supply")),

            // ---
            // Computed,
            // ---
            supply_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("supply_to_circulating_supply_ratio"),
            ),
            halved_supply: BiMap::new_bin(1, MapKind::Computed, &f("halved_supply")),
            halved_supply_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("halved_supply_to_circulating_supply_ratio"),
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
            date,
            is_date_last_block,
            ..
        }: &InsertData,
        state: &SupplyState,
    ) {
        let total_supply = self.supply.height.insert(height, state.supply().to_btc());

        if is_date_last_block {
            self.supply.date.insert(date, total_supply);
        }
    }

    #[allow(unused_variables)]
    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        circulating_supply: &mut BiMap<f64>,
    ) {
        self.supply_to_circulating_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply, circulating_supply);

        self.halved_supply
            .multi_insert_simple_transform(heights, dates, &mut self.supply, &|v| v / 2.0);

        self.halved_supply_to_circulating_supply_ratio
            .multi_insert_simple_transform(
                heights,
                dates,
                &mut self.supply_to_circulating_supply_ratio,
                &|v| v / 2.0,
            );
    }
}

impl AnyDataset for SupplySubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
