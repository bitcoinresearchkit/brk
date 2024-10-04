use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, InsertData, MinInitialStates},
    states::UnrealizedState,
    structs::{AnyBiMap, BiMap},
};

#[derive(Default, Allocative)]
pub struct UnrealizedSubDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    supply_in_profit: BiMap<f64>,
    unrealized_profit: BiMap<f32>,
    unrealized_loss: BiMap<f32>,

    // Computed
    supply_in_loss: BiMap<f64>,
    negative_unrealized_loss: BiMap<f32>,
    net_unrealized_profit_and_loss: BiMap<f32>,
    net_unrealized_profit_and_loss_to_market_cap_ratio: BiMap<f32>,
    supply_in_profit_to_own_supply_ratio: BiMap<f64>,
    supply_in_profit_to_circulating_supply_ratio: BiMap<f64>,
    supply_in_loss_to_own_supply_ratio: BiMap<f64>,
    supply_in_loss_to_circulating_supply_ratio: BiMap<f64>,
}

impl UnrealizedSubDataset {
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

            supply_in_profit: BiMap::new_bin(1, &f("supply_in_profit")),
            supply_in_loss: BiMap::new_bin(1, &f("supply_in_loss")),
            unrealized_profit: BiMap::new_bin(1, &f("unrealized_profit")),
            unrealized_loss: BiMap::new_bin(1, &f("unrealized_loss")),
            negative_unrealized_loss: BiMap::new_bin(1, &f("negative_unrealized_loss")),
            net_unrealized_profit_and_loss: BiMap::new_bin(1, &f("net_unrealized_profit_and_loss")),
            net_unrealized_profit_and_loss_to_market_cap_ratio: BiMap::new_bin(
                2,
                &f("net_unrealized_profit_and_loss_to_market_cap_ratio"),
            ),
            supply_in_profit_to_own_supply_ratio: BiMap::new_bin(
                1,
                &f("supply_in_profit_to_own_supply_ratio"),
            ),
            supply_in_profit_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                &f("supply_in_profit_to_circulating_supply_ratio"),
            ),
            supply_in_loss_to_own_supply_ratio: BiMap::new_bin(
                1,
                &f("supply_in_loss_to_own_supply_ratio"),
            ),
            supply_in_loss_to_circulating_supply_ratio: BiMap::new_bin(
                1,
                &f("supply_in_loss_to_circulating_supply_ratio"),
            ),
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
            ..
        }: &InsertData,
        block_state: &UnrealizedState,
        date_state: &Option<UnrealizedState>,
    ) {
        self.supply_in_profit
            .height
            .insert(height, block_state.supply_in_profit().to_btc());

        self.unrealized_profit
            .height
            .insert(height, block_state.unrealized_profit().to_dollar() as f32);

        self.unrealized_loss
            .height
            .insert(height, block_state.unrealized_loss().to_dollar() as f32);

        if is_date_last_block {
            let date_state = date_state.as_ref().unwrap();

            self.supply_in_profit
                .date
                .insert(date, date_state.supply_in_profit().to_btc());

            self.unrealized_profit
                .date
                .insert(date, date_state.unrealized_profit().to_dollar() as f32);

            self.unrealized_loss
                .date
                .insert(date, date_state.unrealized_loss().to_dollar() as f32);
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        own_supply: &mut BiMap<f64>,
        circulating_supply: &mut BiMap<f64>,
        market_cap: &mut BiMap<f32>,
    ) {
        self.supply_in_loss.multi_insert_subtract(
            heights,
            dates,
            own_supply,
            &mut self.supply_in_profit,
        );

        self.negative_unrealized_loss.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.unrealized_loss,
            &|v| v * -1.0,
        );

        self.net_unrealized_profit_and_loss.multi_insert_subtract(
            heights,
            dates,
            &mut self.unrealized_profit,
            &mut self.unrealized_loss,
        );

        self.net_unrealized_profit_and_loss_to_market_cap_ratio
            .multi_insert_percentage(
                heights,
                dates,
                &mut self.net_unrealized_profit_and_loss,
                market_cap,
            );

        self.supply_in_profit_to_own_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_profit, own_supply);

        self.supply_in_profit_to_circulating_supply_ratio
            .multi_insert_percentage(
                heights,
                dates,
                &mut self.supply_in_profit,
                circulating_supply,
            );

        self.supply_in_loss_to_own_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_loss, own_supply);

        self.supply_in_loss_to_circulating_supply_ratio
            .multi_insert_percentage(heights, dates, &mut self.supply_in_loss, circulating_supply);
    }
}

impl AnyDataset for UnrealizedSubDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_in_profit,
            &self.unrealized_profit,
            &self.unrealized_loss,
        ]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.supply_in_profit,
            &mut self.unrealized_profit,
            &mut self.unrealized_loss,
        ]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.supply_in_loss,
            &self.negative_unrealized_loss,
            &self.net_unrealized_profit_and_loss,
            &self.net_unrealized_profit_and_loss_to_market_cap_ratio,
            &self.supply_in_profit_to_own_supply_ratio,
            &self.supply_in_profit_to_circulating_supply_ratio,
            &self.supply_in_loss_to_own_supply_ratio,
            &self.supply_in_loss_to_circulating_supply_ratio,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.supply_in_loss,
            &mut self.negative_unrealized_loss,
            &mut self.net_unrealized_profit_and_loss,
            &mut self.net_unrealized_profit_and_loss_to_market_cap_ratio,
            &mut self.supply_in_profit_to_own_supply_ratio,
            &mut self.supply_in_profit_to_circulating_supply_ratio,
            &mut self.supply_in_loss_to_own_supply_ratio,
            &mut self.supply_in_loss_to_circulating_supply_ratio,
        ]
    }
}
