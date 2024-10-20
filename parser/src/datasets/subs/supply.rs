use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::SupplyState,
    structs::{AnyBiMap, BiMap, Config},
};

#[derive(Default, Allocative)]
pub struct SupplySubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub supply: BiMap<f64>,

    // Computed
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

            supply: BiMap::new_bin(1, &f("supply")),
            supply_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                &f("supply_to_circulating_supply_ratio"),
            ),
            halved_supply: BiMap::new_bin(1, &f("halved_supply")),
            halved_supply_to_circulating_supply_ratio: BiMap::new_bin(
                1,
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

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.supply]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![&mut self.supply]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_to_circulating_supply_ratio,
            &self.halved_supply,
            &self.halved_supply_to_circulating_supply_ratio,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.supply_to_circulating_supply_ratio,
            &mut self.halved_supply,
            &mut self.halved_supply_to_circulating_supply_ratio,
        ]
    }
}
